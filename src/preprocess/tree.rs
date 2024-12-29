use ast_grep_core::matcher::KindMatcher;
use ast_grep_core::source::Edit;
use ast_grep_core::{AstGrep, Language, StrDoc};

pub struct Tree<L: Language> {
    lang: L,
    ag: AstGrep<StrDoc<L>>,
}

impl<L: Language + Copy> Tree<L> {
    pub fn new<S: AsRef<str>>(src: S, lang: L) -> Self {
        Self {
            ag: lang.ast_grep(src.as_ref()),
            lang,
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
                self.ag = self.lang.ast_grep(&new_content);
                self
            }
        }
    }

    pub fn remove_comments<S: AsRef<str>>(&mut self, kind: S) -> &mut Self {
        let pat = KindMatcher::new(kind.as_ref(), self.lang);
        let edits = self.ag.root().replace_all(&pat, "");
        self.apply_edits(edits)
    }

    pub fn subst_ident<S: AsRef<str>>(&mut self, kind: S, to: &str) -> &mut Self {
        let pat = KindMatcher::new(kind.as_ref(), self.lang);
        let edits = self.ag.root().replace_all(&pat, to);
        self.apply_edits(edits)
    }

    pub fn subst_string<S: AsRef<str>>(&mut self, kind: S, to: &str) -> &mut Self {
        let pat = KindMatcher::new(kind.as_ref(), self.lang);
        let edits = self.ag.root().replace_all(&pat, to);
        self.apply_edits(edits)
    }

    pub fn source(&self) -> &str {
        self.ag.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast_grep_language::Python;

    #[test]
    fn var_subst() {
        let mut tree = Tree::new("def f(a, b, c):\n\ta = 1", Python);
        tree.subst_ident("identifier", "v");
        assert_eq!(tree.source(), "def v(v, v, v):\n\tv = 1");
    }
}
