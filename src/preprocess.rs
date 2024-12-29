use std::borrow::Cow;

pub mod python;
mod tree;

pub trait Preprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str>;
}
