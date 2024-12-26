pub mod arg;
pub mod fingerprint;
pub mod preprocess;
pub mod winnow;

use crate::fingerprint::FingerPrint;
use clap::Parser;
use walkdir::WalkDir;

fn main() {
    let args = arg::Args::parse();

    for entry in WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let f = std::fs::read_to_string(path).expect("should read file");
        let footprint = FingerPrint::new(
            f,
            preprocess::python::PythonPreprocessor,
            fingerprint::FingerPrintConfig::default(),
        );
        println!("processing: {}", path.display());
    }
}
