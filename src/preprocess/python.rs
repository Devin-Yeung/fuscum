use crate::preprocess::tree::Tree;
use crate::preprocess::Preprocessor;
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

impl Preprocessor for PythonPreprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str> {
        let mut tree = Tree::new(src, Python);

        if self.remove_comments {
            tree.remove_comments("comment");
        }

        self.subst_var
            .as_ref()
            .map(|v| tree.subst_ident("identifier", v));

        self.subst_string
            .as_ref()
            .map(|v| tree.subst_string("string", v));
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
    fn preprocess() {
        let src = "def f(a, b, c):\n\ta = \"hello\" # comment";
        let pp = PythonPreprocessor::builder()
            .subst_var("v")
            .subst_string("\"s\"")
            .remove_comments(true)
            .build();
        assert_eq!(pp.preprocess(&src), "defv(v,v,v):v=\"s\"");
    }
}
