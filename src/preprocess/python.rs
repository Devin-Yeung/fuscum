use crate::preprocess::Preprocessor;
use ast_grep_core::matcher::KindMatcher;
use ast_grep_core::source::Edit;
use ast_grep_core::{AstGrep, Language, Pattern, StrDoc};
use ast_grep_language::Python;
use std::borrow::Cow;

pub struct PythonPreprocessor;
pub struct PyTree {
    ag: AstGrep<StrDoc<Python>>,
}

impl PyTree {
    pub fn new<S: AsRef<str>>(src: S) -> Self {
        Self {
            ag: Python.ast_grep(src.as_ref()),
        }
    }

    pub fn apply_edits(&mut self, edits: Vec<Edit<String>>) -> &mut Self {
        edits
            .into_iter()
            .rev()
            .fold(&mut self.ag, |root, edit| root.edit(edit).unwrap());
        self
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

    pub fn source(&self) -> &str {
        self.ag.source()
    }
}

impl Preprocessor for PythonPreprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str> {
        let mut tree = PyTree::new(src);
        tree.subst_ident("v").remove_comments();
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
        let src = "def f(a, b, c):\n\ta = 1";
        let pp = PythonPreprocessor.preprocess(&src);
        assert_eq!(pp, "defv(v,v,v):v=1");
    }
}
