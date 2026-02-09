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
            fn preprocess(&self, src: &str) -> Cow<'_, str> {
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

impl_lang_preprocessor!(
    CppPreprocessor,
    lang => ast_grep_language::Cpp,
    ident => "identifier",
    string => "string_literal",
    comment => "comment"
);

impl_lang_preprocessor!(
    JavaScriptPreprocessor,
    lang => ast_grep_language::JavaScript,
    ident => "identifier",
    string => "string",
    comment => "comment"
);

impl_lang_preprocessor!(
    TypeScriptPreprocessor,
    lang => ast_grep_language::TypeScript,
    ident => "identifier",
    string => "string",
    comment => "comment"
);

impl_lang_preprocessor!(
    JavaPreprocessor,
    lang => ast_grep_language::Java,
    ident => "identifier",
    string => "string_literal",
    comment => "comment"
);

impl_lang_preprocessor!(
    GoPreprocessor,
    lang => ast_grep_language::Go,
    ident => "identifier",
    string => "interpreted_string_literal",
    comment => "comment"
);

impl_lang_preprocessor!(
    RustPreprocessor,
    lang => ast_grep_language::Rust,
    ident => "identifier",
    string => "string_literal",
    comment => "line_comment"
);

impl_lang_preprocessor!(
    RubyPreprocessor,
    lang => ast_grep_language::Ruby,
    ident => "identifier",
    string => "string",
    comment => "comment"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python() {
        let src = include_str!("../../../../fixtures/langs/python.py");
        let pp = PythonPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn c() {
        let src = include_str!("../../../../fixtures/langs/c.c");
        let pp = CPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn cpp() {
        let src = include_str!("../../../../fixtures/langs/cpp.cpp");
        let pp = CppPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn javascript() {
        let src = include_str!("../../../../fixtures/langs/javascript.js");
        let pp = JavaScriptPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn typescript() {
        let src = include_str!("../../../../fixtures/langs/typescript.ts");
        let pp = TypeScriptPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn java() {
        let src = include_str!("../../../../fixtures/langs/java.java");
        let pp = JavaPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn go() {
        let src = include_str!("../../../../fixtures/langs/go.go");
        let pp = GoPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn rust_lang() {
        let src = include_str!("../../../../fixtures/langs/rust.rs");
        let pp = RustPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }

    #[test]
    fn ruby() {
        let src = include_str!("../../../../fixtures/langs/ruby.rb");
        let pp = RubyPreprocessor::default();
        let res = pp.preprocess(src);
        insta::assert_snapshot!(&format!("{src}\n\n\n{res}"))
    }
}
