use std::borrow::Cow;

mod lang;
mod tree;

pub use lang::*;

pub trait Preprocessor {
    fn preprocess(&self, src: &str) -> Cow<str>;
}

impl Preprocessor for Box<dyn Preprocessor> {
    fn preprocess(&self, src: &str) -> Cow<str> {
        self.as_ref().preprocess(src)
    }
}
