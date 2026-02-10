mod analysis;
mod arg;
mod discovery;
mod output;
mod summary;
mod visual;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use arg::{Cli, Commands};

fn run(args: arg::Args) -> Result<()> {
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan(args) => {
            run(args)?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, &name, &mut std::io::stdout());
        }
    }

    Ok(())
}
