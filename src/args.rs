//! Handle `cargo info` arguments
use clap::Parser;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum Command {
    /// Query crates.io for crates details.
    #[command(name = "info")]
    Info(Args),
}

#[derive(Debug, Parser)]
pub struct Args {
    /// Crate name to be queried
    #[arg(name = "crate", required = true)]
    pub crates: Vec<String>,

    /// Report documentation URL
    #[arg(short)]
    pub documentation: bool,

    /// Report number of crate downloads
    #[arg(short = 'D')]
    pub downloads: bool,

    /// Report home page URL
    #[arg(short = 'H')]
    pub homepage: bool,

    /// Report crate repository URL
    #[arg(short)]
    pub repository: bool,

    /// Report crate keywords
    #[arg(short)]
    pub keywords: bool,

    /// Report raw JSON data from crates.io
    #[arg(
        short,
        conflicts_with_all = &["documentation", "downloads", "homepage", "repository", "keywords"]
    )]
    pub json: bool,

    /// Report more details
    #[arg(short)]
    pub verbose: bool,

    /// Report version history of the crate (5 last versions), twice for full history
    #[arg(short = 'V', action = clap::ArgAction::Count)]
    pub versions: u8,

    /// Include prerelease versions when fetching from crates.io (e.g.
    /// '0.6.0-alpha').
    #[arg(short)]
    pub allow_prerelease: bool,

    /// Run without accessing the network
    #[arg(short)]
    pub offline: bool,
}

#[cfg(test)]
impl Default for Args {
    fn default() -> Args {
        Args {
            crates: vec!["demo".to_owned()],
            documentation: true,
            downloads: true,
            homepage: true,
            repository: true,
            keywords: true,
            json: false,
            verbose: false,
            versions: 5,
            allow_prerelease: true,
            offline: false,
        }
    }
}
