pub mod arg;
pub mod fingerprint;
pub mod preprocess;
pub mod winnow;

use crate::fingerprint::FingerPrint;
use clap::Parser;
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let args = arg::Args::parse();

    let entries = WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect::<Vec<_>>();

    let finger_prints = entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            let f = std::fs::read_to_string(path).expect("should read file");
            println!("processing: {}", path.display());
            let footprint = FingerPrint::new(
                f,
                preprocess::python::PythonPreprocessor,
                fingerprint::FingerPrintConfig::default(),
            );
            footprint
        })
        .collect::<Vec<_>>();
}
