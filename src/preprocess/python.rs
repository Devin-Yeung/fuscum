use ast_grep_core::{AstGrep, Language, Pattern, StrDoc};
use ast_grep_language::Python;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_preprocessor() {
        let mut pp = PythonPreprocessor::new("def f(a, b, c):\n\ta = 1");
        pp.subst_identifier("v");
        assert_eq!(pp.source(), "def v(v, v, v):\n\tv = 1");
    }
}
