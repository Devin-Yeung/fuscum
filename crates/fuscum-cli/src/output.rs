use std::fs;

use anyhow::{Context, Result};
use rinja::Template;

use crate::arg::Args;
use crate::summary::Summaries;
use crate::visual::NetworkTemplate;

pub struct OutputHandler {
    args: Args,
}

impl OutputHandler {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn print_table(&self, summaries: &Summaries) -> Result<()> {
        println!("{:<30} {:>10}", "File", "Max Score");
        println!("{}", "-".repeat(42));
        for s in summaries.iter() {
            println!("{:<30} {:>10.4}", s.base, s.max_score);
            for p in &s.against {
                println!("  {:<28} {:>10.4}", p.against, p.score);
            }
        }
        Ok(())
    }

    pub fn write_json(&self, summaries: &Summaries) -> Result<()> {
        if let Some(json_path) = &self.args.json {
            let json = serde_json::to_string_pretty(&summaries.0)?;
            fs::write(json_path, json)
                .with_context(|| format!("failed to write {}", json_path.display()))?;
            println!("\nJSON written to {}", json_path.display());
        }
        Ok(())
    }

    pub fn write_network(&self, summaries: &Summaries) -> Result<()> {
        if let Some(network_path) = &self.args.network {
            let template = NetworkTemplate::new(summaries, self.args.threshold);
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
}
