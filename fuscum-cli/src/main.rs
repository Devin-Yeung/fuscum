mod arg;
mod submission;
mod summary;

use crate::submission::Submission;
use crate::summary::{SourceSummary, Summary};
use clap::Parser;
use fuscum::doc::MultiDoc;
use glob::glob;
use std::cmp::Ordering;
// use rayon::prelude::*;
// use std::io::Read;

fn main() {
    let args = arg::Args::parse();

    let docs = glob(&args.pat)
        .expect("Failed to read glob pattern")
        .filter_map(|glob| {
            glob.ok().map(|path| {
                let doc: MultiDoc = Submission::new(path).unwrap().into();
                doc
            })
        })
        .collect::<Vec<_>>();

    let mut results = docs
        .iter()
        .enumerate()
        .map(|(i, base)| {
            let mut sims = docs
                .iter()
                .enumerate()
                .filter_map(|(j, against)| match i.cmp(&j) {
                    Ordering::Equal => None,
                    _ => Some(base.similarity(against)),
                })
                .collect::<Vec<_>>();
            // take top 3 similar docs
            sims.sort_by(|a, b| b.score().total_cmp(&a.score()));
            let against: Vec<_> = sims.into_iter().take(3).map(|s| s.into()).collect();
            Summary {
                base: base.name().to_string(),
                max_score: against
                    .iter()
                    .map(|s: &SourceSummary| s.score)
                    .reduce(f32::max)
                    .unwrap(),
                against,
            }
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| b.score().total_cmp(&a.score()));
    let results = results
        .into_iter()
        .filter(|r| r.max_score > args.threshold)
        .collect::<Vec<_>>();

    for result in &results {
        println!("{}: {}", result.base, result.score());
    }

    println!("{} in total", results.len());
    // write to a json file
    let json = serde_json::to_string_pretty(&results).expect("should serialize to json");
    std::fs::write("results.json", json).expect("should write to file");
}
