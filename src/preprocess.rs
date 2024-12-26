use std::borrow::Cow;

pub mod python;

pub trait Preprocessor {
    fn preprocess<S: AsRef<str>>(src: &S) -> Cow<str>;
}
