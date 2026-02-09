mod arg;
mod summary;
mod visual;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use fuscum::fingerprint::{FingerPrint, FingerPrintConfig, FingerPrintGenerator, WithFingerprint};
use fuscum::kgram::default_rolling_kgram;
use rayon::prelude::*;
use rinja::Template;

use crate::summary::{PairSummary, Summary};
use crate::visual::NetworkTemplate;

fn main() -> Result<()> {
    let args = arg::Args::parse();

    // Glob for files and generate fingerprints
    let full_pat = args.dir.join(&args.pat);
    let full_pat = full_pat.to_string_lossy();
    let paths: Vec<_> = glob::glob(&full_pat)
        .context("invalid glob pattern")?
        .filter_map(Result::ok)
        .collect();

    let fingerprints: Vec<(String, FingerPrint)> = paths
        .par_iter()
        .map(|path| {
            let preprocessor = args.lang.preprocessor();
            let gen = FingerPrintGenerator {
                config: FingerPrintConfig::default(),
                preprocessor,
                kgram: default_rolling_kgram(),
            };
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            println!("Processing {}", name);
            let src = fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))
                .unwrap();
            let fp = gen.generate(&src);
            (name, fp)
        })
        .collect();

    // Compute similarities in parallel
    let mut summaries: Vec<Summary> = fingerprints
        .par_iter()
        .map(|(name, fp)| {
            let mut pairs: Vec<PairSummary> = fingerprints
                .iter()
                .filter(|(other_name, _)| other_name != name)
                .map(|(other_name, other_fp)| PairSummary {
                    against: other_name.clone(),
                    score: fp.similarity(other_fp),
                })
                .collect();

            pairs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            pairs.truncate(3);

            let max_score = pairs.first().map(|p| p.score).unwrap_or(0.0);

            Summary {
                base: name.clone(),
                max_score,
                against: pairs,
            }
        })
        .collect();

    summaries.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap());
    summaries.retain(|s| s.max_score >= args.threshold);

    // Print human-readable table
    println!("{:<30} {:>10}", "File", "Max Score");
    println!("{}", "-".repeat(42));
    for s in &summaries {
        println!("{:<30} {:>10.4}", s.base, s.max_score);
        for p in &s.against {
            println!("  {:<28} {:>10.4}", p.against, p.score);
        }
    }

    // Optional JSON output
    if let Some(json_path) = &args.json {
        let json = serde_json::to_string_pretty(&summaries)?;
        fs::write(json_path, json)
            .with_context(|| format!("failed to write {}", json_path.display()))?;
        println!("\nJSON written to {}", json_path.display());
    }

    // Optional network visualization
    if let Some(network_path) = &args.network {
        let template = NetworkTemplate::new(&summaries, args.threshold);
        let rendered = template
            .render()
            .context("failed to render network template")?;
        fs::write(network_path, rendered)
            .with_context(|| format!("failed to write {}", network_path.display()))?;
        println!(
            "Network visualization written to {}",
            network_path.display()
        );
    }

    Ok(())
}
