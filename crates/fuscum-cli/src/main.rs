mod analysis;
mod arg;
mod discovery;
mod output;
mod summary;
mod visual;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = arg::Args::parse();

    // Discover files and generate fingerprints
    let discovery = discovery::FileDiscovery::new(args.clone());
    let paths = discovery.discover_files()?;
    let submissions = discovery.generate_fingerprints(&paths)?;

    // Analyze similarities
    let analyzer = analysis::SimilarityAnalyzer::new(args.threshold, args.top_k);
    let summaries = analyzer.analyze_fingerprints(&submissions)?;

    // Handle output
    let output = output::OutputHandler::new(args);
    output.print_table(&summaries)?;
    output.write_json(&summaries)?;
    output.write_network(&summaries)?;

    Ok(())
}
