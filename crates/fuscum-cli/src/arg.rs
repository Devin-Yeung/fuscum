use std::path::PathBuf;

use clap::Parser;
use fuscum::preprocess::{
    CPreprocessor, CppPreprocessor, GoPreprocessor, JavaPreprocessor, JavaScriptPreprocessor,
    PythonPreprocessor, RubyPreprocessor, RustPreprocessor, TypeScriptPreprocessor,
};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Lang {
    Python,
    C,
    Cpp,
    JavaScript,
    TypeScript,
    Java,
    Go,
    Rust,
    Ruby,
}

impl Lang {
    pub fn preprocessor(&self) -> Box<dyn fuscum::preprocess::Preprocessor> {
        match self {
            Lang::Python => Box::new(PythonPreprocessor::default()),
            Lang::C => Box::new(CPreprocessor::default()),
            Lang::Cpp => Box::new(CppPreprocessor::default()),
            Lang::JavaScript => Box::new(JavaScriptPreprocessor::default()),
            Lang::TypeScript => Box::new(TypeScriptPreprocessor::default()),
            Lang::Java => Box::new(JavaPreprocessor::default()),
            Lang::Go => Box::new(GoPreprocessor::default()),
            Lang::Rust => Box::new(RustPreprocessor::default()),
            Lang::Ruby => Box::new(RubyPreprocessor::default()),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to search for source files
    #[arg(default_value = ".")]
    pub dir: PathBuf,

    /// The glob pattern to match source files within the directory
    #[arg(long)]
    pub pat: String,

    /// The language of the source files
    #[arg(long)]
    pub lang: Lang,

    /// The threshold to consider two files similar
    #[arg(long, default_value = "0.4")]
    pub threshold: f32,

    /// Write JSON results to this file
    #[arg(long)]
    pub json: Option<PathBuf>,

    /// Write network visualization HTML to this file
    #[arg(long)]
    pub network: Option<PathBuf>,
}
