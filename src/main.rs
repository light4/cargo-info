use std::fmt;

use clap::Parser;
use color_eyre::Result;

use crate::args::{Args, Command};

mod args;
mod crates;

static USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

#[derive(Debug, PartialEq)]
enum Flag {
    Repository,
    Documentation,
    Downloads,
    Homepage,
    Default,
}

#[derive(Debug)]
struct Report {
    flags: Vec<Flag>,
    verbose: bool,
    json: bool,
    versions: usize,
    keywords: bool,
}

impl Report {
    pub fn new(args: &Args) -> Self {
        let mut flags: Vec<Flag> = vec![];
        if args.repository {
            flags.push(Flag::Repository);
        }
        if args.documentation {
            flags.push(Flag::Documentation);
        }
        if args.downloads {
            flags.push(Flag::Downloads);
        }
        if args.homepage {
            flags.push(Flag::Homepage);
        }

        if flags.is_empty() {
            flags.push(Flag::Default);
        }
        let versions = match args.versions {
            0 => 0,
            1 => 5,
            _ => usize::max_value(),
        };

        Report {
            flags,
            verbose: args.verbose,
            json: args.json,
            versions,
            keywords: args.keywords,
        }
    }

    pub fn report(&self, name: &str) -> Result<String> {
        let krate_detail = get_crate(name)?;
        let krate_json = json::parse(&krate_detail).expect("get crate parse json error");

        if self.json {
            return Ok(krate_json.pretty(2));
        }

        let mut output = String::new();
        let krate = crates::Crate::new(&krate_json);
        if self.versions > 0 {
            output = output + &self.report_versions(&krate, self.versions);
        } else if self.keywords {
            output = output + &self.report_keywords(&krate);
        } else {
            output = output + &self.report_crate(&krate);
        }
        Ok(output)
    }

    pub fn report_crate(&self, krate: &crates::Crate) -> String {
        let mut output = String::new();
        for flag in &self.flags {
            output = output
                + &match *flag {
                    Flag::Repository => krate.print_repository(self.verbose),
                    Flag::Documentation => krate.print_documentation(self.verbose),
                    Flag::Downloads => krate.print_downloads(self.verbose),
                    Flag::Homepage => krate.print_homepage(self.verbose),
                    Flag::Default => reportv(krate, self.verbose),
                }
        }
        output
    }

    pub fn report_versions(&self, krate: &crates::Crate, limit: usize) -> String {
        if limit > 0 {
            krate.print_last_versions(limit, self.verbose)
        } else {
            String::new()
        }
    }

    pub fn report_keywords(&self, krate: &crates::Crate) -> String {
        krate.print_keywords(self.verbose)
    }
}

fn reportv(krate: &crates::Crate, verbose: bool) -> String {
    if verbose {
        format!("{krate:#}")
    } else {
        format!("{krate}")
    }
}

fn get_crate(krate: &str) -> Result<String> {
    let body = ureq::get(&format!("https://crates.io/api/v1/crates/{krate}"))
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_string()?;

    Ok(body)
}

fn print_report<T>(r: Result<T>)
where
    T: fmt::Display,
{
    match r {
        Ok(text) => println!("\n{text}\n"),
        Err(err) => eprintln!("\n{err}\n"),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Command = Command::parse();
    let Command::Info(args) = args;

    let rep = Report::new(&args);
    for krate in args.crates {
        print_report(rep.report(&krate));
    }

    Ok(())
}
