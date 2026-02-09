use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use fuscum::fingerprint::{FingerPrint, FingerPrintConfig, FingerPrintGenerator};
use fuscum::kgram::default_rolling_kgram;
use rayon::prelude::*;

use crate::arg::Args;

pub struct Submission {
    pub name: String,
    pub fingerprint: FingerPrint,
}

pub struct FileDiscovery {
    args: Args,
}

impl FileDiscovery {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn discover_files(&self) -> Result<Vec<PathBuf>> {
        let full_pat = self.args.dir.join(&self.args.pat);
        let full_pat = full_pat.to_string_lossy();
        let paths: Vec<_> = glob::glob(&full_pat)
            .context("invalid glob pattern")?
            .filter_map(Result::ok)
            .collect();
        Ok(paths)
    }

    pub fn generate_fingerprints(&self, files: &[PathBuf]) -> Result<Vec<Submission>> {
        let submissions: Vec<Submission> = files
            .par_iter()
            .map(|path| {
                let preprocessor = self.args.lang.preprocessor();
                let gen = FingerPrintGenerator {
                    config: FingerPrintConfig::new(self.args.kgram_size, self.args.window_size),
                    preprocessor,
                    kgram: default_rolling_kgram(),
                };
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                println!("Processing {}", name);
                let src = fs::read_to_string(path)
                    .with_context(|| format!("failed to read {}", path.display()))
                    .unwrap();
                let fingerprint = gen.generate(&src);
                Submission { name, fingerprint }
            })
            .collect();
        Ok(submissions)
    }
}
