use std::fmt;

use async_std::{eprintln, println};
use structopt::StructOpt;
use surf;
use anyhow::{anyhow, Result};

use crate::args::{Args, Command};

mod args;
mod crates;

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

    pub async fn report(&self, name: &str) -> Result<String> {
        let krate_detail = get_crate(name).await?;
        let krate_json = json::parse(&krate_detail).expect("get crate parse json error");

        if self.json {
            return self.report_json(&krate_json).await;
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

    pub async fn report_json(&self, krate_json: &json::JsonValue) -> Result<String> {
        let mut output = String::new();
        if self.verbose {
            output = output + &format!("{:#}", krate_json);
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
        format!("{:#}", krate)
    } else {
        format!("{}", krate)
    }
}

async fn get_crate(krate: &str) -> Result<String> {
    let r = surf::get(&format!("https://crates.io/api/v1/crates/{}", krate))
        .recv_string()
        .await;
    match r {
        Ok(s) => Ok(s),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

async fn print_report<T>(r: Result<T>)
where
    T: fmt::Display,
{
    match r {
        Ok(text) => println!("\n{}\n", text).await,
        Err(err) => eprintln!("\n{}\n", err).await,
    }
}

#[async_std::main]
async fn main() {
    let args: Command = Command::from_args();
    let Command::Info(args) = args;

    let rep = Report::new(&args);
    for krate in args.crates {
        print_report(rep.report(&krate).await).await;
    }
}
