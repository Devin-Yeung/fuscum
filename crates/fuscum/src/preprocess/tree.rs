use ast_grep_core::matcher::KindMatcher;
use ast_grep_core::source::Edit;
use ast_grep_core::tree_sitter::StrDoc;
use ast_grep_core::{AstGrep, Language};
use ast_grep_language::LanguageExt;

pub struct Tree<L: Language + LanguageExt> {
    lang: L,
    ag: AstGrep<StrDoc<L>>,
}

impl<L: Language + Copy + LanguageExt> Tree<L> {
    pub fn new<S: AsRef<str>>(src: S, lang: L) -> Self {
        Self {
            ag: lang.ast_grep(src.as_ref()),
            lang,
        }
    }

    pub fn apply_edit_helper(&self, edits: Vec<Edit<String>>) -> String {
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
        let edits = self
            .ag
            .root()
            .find_all(&pat)
            .map(|m| m.remove())
            .collect::<Vec<_>>();
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

    /// Single-parse variant of `remove_comments` + `subst_ident` + `subst_string`.
    ///
    /// All edits are collected against the original tree and applied to the text
    /// once, so the file is parsed once and never re-parsed.
    /// When edits overlap (e.g. an identifier inside an f-string that is replaced as a
    /// whole), the outermost edit wins, which matches the sequential behaviour.
    pub fn rewrite_single_pass(
        &self,
        comment: Option<&str>,
        ident: Option<(&str, &str)>,
        string: Option<(&str, &str)>,
    ) -> String {
        let root = self.ag.root();
        let mut edits: Vec<Edit<String>> = Vec::new();
        if let Some(kind) = comment {
            let pat = KindMatcher::new(kind, self.lang);
            edits.extend(root.find_all(&pat).map(|m| m.remove()));
        }
        if let Some((kind, to)) = ident {
            let pat = KindMatcher::new(kind, self.lang);
            edits.extend(root.replace_all(&pat, to));
        }
        if let Some((kind, to)) = string {
            let pat = KindMatcher::new(kind, self.lang);
            edits.extend(root.replace_all(&pat, to));
        }

        // Outermost-first: sort by start, longer first on ties. Tree nodes either
        // are disjoint or nest, so an edit either starts after the kept edit ends
        // (keep it) or lies inside it (drop it, the outer replacement covers it).
        // Partial overlap should be impossible; assert it so it cannot pass silently.
        edits.sort_by(|a, b| {
            a.position
                .cmp(&b.position)
                .then(b.deleted_length.cmp(&a.deleted_length))
        });
        let mut kept: Vec<Edit<String>> = Vec::with_capacity(edits.len());
        // end of the outermost kept edit
        let mut end = 0usize;
        for e in edits {
            let e_end = e.position + e.deleted_length;
            if e.position >= end {
                end = e_end;
                kept.push(e);
            } else {
                debug_assert!(
                    e_end <= end,
                    "partially overlapping edits: {:?} and {}..{}",
                    kept.last()
                        .map(|k| k.position..k.position + k.deleted_length),
                    e.position,
                    e_end
                );
            }
        }
        if kept.is_empty() {
            return self.ag.source().to_string();
        }
        // The result is returned as text, so no re-parse is needed.
        self.apply_edit_helper(kept)
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
