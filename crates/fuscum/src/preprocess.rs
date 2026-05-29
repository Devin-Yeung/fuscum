use std::borrow::Cow;

#[cfg(feature = "ast")]
mod lang;
#[cfg(feature = "ast")]
mod tree;

#[cfg(feature = "ast")]
pub use lang::*;

pub trait Preprocessor {
    fn preprocess(&self, src: &str) -> Cow<'_, str>;
}

impl Preprocessor for Box<dyn Preprocessor> {
    fn preprocess(&self, src: &str) -> Cow<'_, str> {
        self.as_ref().preprocess(src)
    }
}
