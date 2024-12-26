use std::borrow::Cow;

pub mod python;

pub trait Preprocessor {
    fn preprocess<S: AsRef<str>>(&self, src: &S) -> Cow<str>;
}
