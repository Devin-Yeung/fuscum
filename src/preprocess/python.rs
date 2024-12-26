use crate::preprocess::Preprocessor;
use ast_grep_core::{AstGrep, Language, Pattern, StrDoc};
use ast_grep_language::Python;
use std::borrow::Cow;

pub struct PythonPreprocessor {
    ag: AstGrep<StrDoc<Python>>,
}

impl PythonPreprocessor {
    pub fn new<S: AsRef<str>>(src: S) -> Self {
        Self {
            ag: Python.ast_grep(src.as_ref()),
        }
    }

    pub fn subst_identifier(&mut self, to: &str) {
        let pat = Pattern::contextual("$V", "identifier", Python).expect("should parse");

        let edits = self.ag.root().replace_all(&pat, to);

        edits
            .into_iter()
            .fold(&mut self.ag, |root, edit| root.edit(edit).unwrap());
    }

    pub fn source(&self) -> &str {
        self.ag.source()
    }
}

impl Preprocessor for PythonPreprocessor {
    fn preprocess<S: AsRef<str>>(src: &S) -> Cow<str> {
        let mut pp = PythonPreprocessor::new(src.as_ref());
        pp.subst_identifier("v");
        // remove all the whitespace in the source code
        let src = pp
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
        let mut pp = PythonPreprocessor::new("def f(a, b, c):\n\ta = 1");
        pp.subst_identifier("v");
        assert_eq!(pp.source(), "def v(v, v, v):\n\tv = 1");
    }

    #[test]
    fn preprocess() {
        let src = "def f(a, b, c):\n\ta = 1";
        let pp = PythonPreprocessor::preprocess(&src);
        assert_eq!(pp, "defv(v,v,v):v=1");
    }
}
