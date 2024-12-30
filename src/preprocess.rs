use std::borrow::Cow;

mod lang;
mod tree;

pub use lang::*;

pub trait Preprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str>;
}
