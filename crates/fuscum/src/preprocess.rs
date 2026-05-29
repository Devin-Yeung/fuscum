use std::borrow::Cow;

#[cfg(feature = "ast")]
mod lang;
mod text;
#[cfg(feature = "ast")]
mod tree;

#[cfg(feature = "ast")]
pub use lang::*;
pub use text::*;

pub trait Preprocessor {
    fn preprocess<'a>(&self, src: &'a str) -> Cow<'a, str>;
}

impl Preprocessor for Box<dyn Preprocessor> {
    fn preprocess<'a>(&self, src: &'a str) -> Cow<'a, str> {
        self.as_ref().preprocess(src)
    }
}
