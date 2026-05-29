use regex::Regex;
use std::borrow::Cow;

use crate::preprocess::Preprocessor;

/// A no-op preprocessor, which just returns the input string unchanged.
#[derive(Debug, Clone, Default)]
pub struct NoPreprocessor();

impl Preprocessor for NoPreprocessor {
    fn preprocess<'a>(&self, src: &'a str) -> std::borrow::Cow<'a, str> {
        Cow::Borrowed(src)
    }
}

/// Strips all matches of a regex in the text
#[derive(Debug, Clone)]
pub struct RegexPreprocessor {
    regex: Regex,
}

impl RegexPreprocessor {
    pub fn whitespace() -> Self {
        Self {
            regex: Regex::new(r"\s").expect("failed to compile whitespace-capturing regex"),
        }
    }
}

impl Preprocessor for RegexPreprocessor {
    fn preprocess<'a>(&self, src: &'a str) -> std::borrow::Cow<'a, str> {
        self.regex.replace_all(src, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::preprocess::{
        text::{NoPreprocessor, RegexPreprocessor},
        Preprocessor,
    };

    #[test]
    fn no_preprocessor() {
        let preprocessor = NoPreprocessor::default();
        assert_eq!(preprocessor.preprocess("hello world!\n"), "hello world!\n");
    }

    #[test]
    fn whitespace_preprocessor() {
        let preprocessor = RegexPreprocessor::whitespace();
        assert_eq!(preprocessor.preprocess("hello world!\n"), "helloworld!");
    }
}
