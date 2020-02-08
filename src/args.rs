//! Handle `cargo info` arguments
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    /// Query crates.io for crates details.
    #[structopt(name = "info")]
    Info(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Crate name to be queried
    #[structopt(name = "crate", required = true)]
    pub crates: Vec<String>,

    /// Report documentation URL
    #[structopt(short)]
    pub documentation: bool,

    /// Report number of crate downloads
    #[structopt(short = "D")]
    pub downloads: bool,

    /// Report home page URL
    #[structopt(short = "H")]
    pub homepage: bool,

    /// Report crate repository URL
    #[structopt(short)]
    pub repository: bool,

    /// Report crate keywords
    #[structopt(short)]
    pub keywords: bool,

    /// Report raw JSON data from crates.io
    #[structopt(
        short,
        conflicts_with_all = &["documentation", "downloads", "homepage", "repository", "keywords"]
    )]
    pub json: bool,

    /// Report more details
    #[structopt(short)]
    pub verbose: bool,

    /// Report version history of the crate (5 last versions), twice for full history
    #[structopt(short = "V", parse(from_occurrences))]
    pub versions: usize,

    /// Include prerelease versions when fetching from crates.io (e.g.
    /// '0.6.0-alpha').
    #[structopt(short)]
    pub allow_prerelease: bool,

    /// Run without accessing the network
    #[structopt(short)]
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
