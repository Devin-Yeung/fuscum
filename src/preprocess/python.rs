use crate::preprocess::Preprocessor;
use ast_grep_core::matcher::KindMatcher;
use ast_grep_core::source::Edit;
use ast_grep_core::{AstGrep, Language, StrDoc};
use ast_grep_language::Python;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(doc, field_defaults(default, setter(into)))]
pub struct PythonPreprocessor {
    #[builder(setter(
        strip_option,
        doc = "Substitute variable to a certain unified representation"
    ))]
    subst_var: Option<String>,
    #[builder(setter(
        strip_option,
        doc = "Substitute string to a certain unified representation"
    ))]
    subst_string: Option<String>,
    #[builder(default = true, setter(doc = "Remove comments from the source code"))]
    remove_comments: bool,
}

impl Default for PythonPreprocessor {
    fn default() -> Self {
        Self {
            subst_var: Some("v".to_string()),
            subst_string: Some("\"s\"".to_string()),
            remove_comments: true,
        }
    }
}

pub struct PyTree {
    ag: AstGrep<StrDoc<Python>>,
}

impl PyTree {
    pub fn new<S: AsRef<str>>(src: S) -> Self {
        Self {
            ag: Python.ast_grep(src.as_ref()),
        }
    }

    pub fn apply_edit_helper(&mut self, edits: Vec<Edit<String>>) -> String {
        debug_assert_ne!(edits.len(), 0);
        let mut new_content = String::new();
        let old_content = self.ag.root().root().get_text();
        let mut start = 0;
        for diff in edits {
            let range = diff.position..diff.position + diff.deleted_length;
            new_content.push_str(&old_content[start..range.start]);
            let replacement = String::from_utf8(diff.inserted_text).unwrap();
            new_content.push_str(&replacement);
            start = range.end;
        }
        // add trailing statements
        new_content.push_str(&old_content[start..]);
        new_content
    }

    pub fn apply_edits(&mut self, edits: Vec<Edit<String>>) -> &mut Self {
        match edits.len() {
            0 => self,
            _ => {
                let new_content = self.apply_edit_helper(edits);
                self.ag = Python.ast_grep(&new_content);
                self
            }
        }
    }

    pub fn remove_comments(&mut self) -> &mut Self {
        let pat = KindMatcher::new("comment", Python);
        let edits = self.ag.root().replace_all(&pat, "");
        self.apply_edits(edits)
    }

    pub fn subst_ident(&mut self, to: &str) -> &mut Self {
        let pat = KindMatcher::new("identifier", Python);
        let edits = self.ag.root().replace_all(&pat, to);
        self.apply_edits(edits) // TODO: bottleneck, it's too slow
    }

    pub fn subst_string(&mut self, to: &str) -> &mut Self {
        let pat = KindMatcher::new("string", Python);
        let edits = self.ag.root().replace_all(&pat, to);
        self.apply_edits(edits)
    }

    pub fn source(&self) -> &str {
        self.ag.source()
    }
}

impl Preprocessor for PythonPreprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str> {
        let mut tree = PyTree::new(src);

        if self.remove_comments {
            tree.remove_comments();
        }

        self.subst_var
            .as_ref()
            .map(|v| tree.subst_ident(v.as_str()));

        self.subst_string
            .as_ref()
            .map(|v| tree.subst_string(v.as_str()));
        // remove all the whitespace in the source code
        let src = tree
            .source()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        Cow::Owned(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_subst() {
        let mut tree = PyTree::new("def f(a, b, c):\n\ta = 1");
        tree.subst_ident("v");
        assert_eq!(tree.source(), "def v(v, v, v):\n\tv = 1");
    }

    #[test]
    fn preprocess() {
        let src = "def f(a, b, c):\n\ta = \"hello\"";
        let pp = PythonPreprocessor::builder()
            .subst_var("v")
            .subst_string("\"s\"")
            .remove_comments(true)
            .build();
        assert_eq!(pp.preprocess(&src), "defv(v,v,v):v=\"s\"");
    }
}
