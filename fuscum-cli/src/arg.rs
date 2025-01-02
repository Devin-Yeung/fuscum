use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The glob pattern to search for files
    #[arg(long)]
    pub pat: String,
    /// The threshold to consider two files similar
    #[arg(long, default_value = "0.4")]
    pub threshold: f32,
}
