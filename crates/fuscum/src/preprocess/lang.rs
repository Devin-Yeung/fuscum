use crate::preprocess::tree::Tree;
use crate::preprocess::Preprocessor;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

macro_rules! impl_lang_preprocessor {
    (
        $name:ident,
        lang => $lang:expr,
        ident => $identifier_token:expr,
        string => $string_token:expr,
        comment => $comment_token:expr
    ) => {
        #[derive(TypedBuilder)]
        #[builder(doc, field_defaults(default, setter(into)))]
        pub struct $name {
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

        impl Default for $name {
            fn default() -> Self {
                Self {
                    subst_var: Some("v".to_string()),
                    subst_string: Some("\"s\"".to_string()),
                    remove_comments: true,
                }
            }
        }

        impl Preprocessor for $name {
            fn preprocess(&self, src: &str) -> Cow<str> {
                let mut tree = Tree::new(src, $lang);

                if self.remove_comments {
                    tree.remove_comments($comment_token);
                }

                self.subst_var
                    .as_ref()
                    .map(|v| tree.subst_ident($identifier_token, v));

                self.subst_string
                    .as_ref()
                    .map(|v| tree.subst_string($string_token, v));

                let src = tree
                    .source()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>();
                Cow::Owned(src)
            }
        }
    };
}

impl_lang_preprocessor!(
    PythonPreprocessor,
    lang => ast_grep_language::Python,
    ident => "identifier",
    string => "string",
    comment => "comment"
);

impl_lang_preprocessor!(
    CPreprocessor,
    lang => ast_grep_language::C,
    ident => "identifier",
    string => "string_literal",
    comment => "comment"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python() {
        let src = include_str!("../../../../fixtures/langs/python.py");
        let pp = PythonPreprocessor::default();
        let res = pp.preprocess(&src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn c() {
        let src = include_str!("../../../../fixtures/langs/c.c");
        let pp = CPreprocessor::default();
        let res = pp.preprocess(&src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }
}
