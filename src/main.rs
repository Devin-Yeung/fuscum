pub mod arg;
mod doc;
pub mod fingerprint;
pub mod preprocess;
pub mod winnow;

use crate::doc::Doc;
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

    let docs = entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            let f = std::fs::read_to_string(path).expect("should read file");
            println!("processing: {}", path.display());
            let finger_print = FingerPrint::new(
                f,
                preprocess::python::PythonPreprocessor::default(),
                fingerprint::FingerPrintConfig::default(),
            );
            Doc {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                finger_print,
            }
        })
        .collect::<Vec<_>>();

    // compare against all the pair
    let mut results = docs
        .iter()
        .enumerate()
        .map(|(i, doc)| {
            docs.iter()
                .enumerate()
                .map(|(j, other)| {
                    if i != j {
                        let similarity = doc.similarity(other);
                        Some(similarity)
                    } else {
                        None
                    }
                })
                .filter(|x| x.is_some())
                .collect::<Option<Vec<_>>>()
        })
        .filter(|x| x.is_some())
        .flat_map(|x| x.unwrap())
        .collect::<Vec<_>>();
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    results
        .iter()
        .filter(|r| r.score > 0.6)
        .for_each(|r| println!("{r:?}"));
}
