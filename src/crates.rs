use std::fmt;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use chrono_humanize::HumanTime;
use json::JsonValue;

#[derive(Debug)]
struct TimeStamp(Option<DateTime<Local>>);

impl<'a> From<&'a JsonValue> for TimeStamp {
    fn from(jv: &JsonValue) -> Self {
        let parse_naive = |s: &str| s.parse::<NaiveDateTime>();
        let naive_to_local = |n: NaiveDateTime| Local.from_utc_datetime(&n);
        let parse_local = |s: &str| s.parse::<DateTime<Local>>();
        let parse = |s: &str| {
            parse_local(s)
                .or_else(|_| parse_naive(s).map(naive_to_local))
                .ok()
        };
        TimeStamp(jv.as_str().and_then(parse))
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ts) = self.0 {
            if f.alternate() {
                f.pad(&format!("{}", HumanTime::from(ts)))
            } else {
                f.pad(&format!("{}", ts.naive_local()))
            }
        } else {
            f.pad("")
        }
    }
}

#[derive(Debug)]
pub struct Crate {
    krate: Krate,
    versions: JsonValue,
    keywords: JsonValue,
}

#[derive(Debug)]
struct Krate {
    name: String,
    downloads: u64,
    max_version: String,
    description: String,
    documentation: String,
    homepage: String,
    repository: String,
    license: String,
    keywords: Vec<String>,
    features: Vec<String>,
    default_features: Vec<String>,
    created_at: TimeStamp,
    updated_at: TimeStamp,
}

impl Crate {
    pub fn new(json: &JsonValue) -> Self {
        let crate_jv = json["crate"].clone();
        let versions = json["versions"].clone();

        let name = crate_jv["name"].to_string();
        let license = versions[0]["license"].to_string();
        let keywords = crate_jv["keywords"]
            .members()
            .map(|jv| jv.to_string())
            .collect::<Vec<_>>();

        let features = versions[0]["features"]
            .entries()
            .filter_map(|(k, _)| {
                if k != "default" {
                    Some(k.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let default_features = versions[0]["features"]["default"]
            .members()
            .map(|jv| jv.to_string())
            .collect::<Vec<_>>();

        let created_at = TimeStamp::from(&crate_jv["created_at"]);
        let updated_at = TimeStamp::from(&crate_jv["updated_at"]);

        let doc = &crate_jv["documentation"];
        let documentation = if doc.is_empty() {
            format!("https://docs.rs/{}", name)
        } else {
            doc.to_string()
        };

        let krate = Krate {
            name,
            downloads: crate_jv["downloads"].as_u64().unwrap_or_default(),
            max_version: crate_jv["max_version"].to_string(),
            description: crate_jv["description"].to_string(),
            documentation,
            homepage: crate_jv["homepage"].to_string(),
            repository: crate_jv["repository"].to_string(),
            license,
            keywords,
            features,
            default_features,
            created_at,
            updated_at,
        };

        Crate {
            krate,
            versions,
            keywords: json["keywords"].clone(),
        }
    }

    pub fn print_repository(&self, verbose: bool) -> String {
        if verbose {
            format!("{:<16}{}", "Repository:", self.krate.repository)
        } else {
            self.krate.repository.to_string()
        }
    }

    pub fn print_documentation(&self, verbose: bool) -> String {
        if verbose {
            format!("{:<16}{}", "Documentation:", self.krate.documentation)
        } else {
            self.krate.documentation.to_string()
        }
    }

    pub fn print_downloads(&self, verbose: bool) -> String {
        if verbose {
            format!("{:<16}{}", "Downloads:", self.krate.downloads)
        } else {
            self.krate.downloads.to_string()
        }
    }

    pub fn print_homepage(&self, verbose: bool) -> String {
        if verbose {
            format!("{:<16}{}", "Homepage:", self.krate.homepage)
        } else {
            self.krate.homepage.to_string()
        }
    }

    fn print_version(v: &JsonValue, _verbose: bool) -> String {
        let created_at = TimeStamp::from(&v["created_at"]);
        let mut output = format!("{:<16}{:<#16}{:<16}", v["num"], created_at, v["downloads"]);

        if v["yanked"].as_bool() == Some(true) {
            output += "\t\t(yanked)";
        }

        // Consider adding some more useful information in verbose mode
        // if verbose {
        // }

        output + "\n"
    }

    fn print_version_header(_verbose: bool) -> String {
        let output = format!("{:<16}{:<#16}{:<16}\n", "VERSION", "RELEASED", "DOWNLOADS");

        // Consider adding some more useful information in verbose mode
        // if verbose {
        // }

        output + "\n"
    }

    pub fn print_last_versions(&self, limit: usize, verbose: bool) -> String {
        let mut output = Crate::print_version_header(verbose);
        for version in self.versions.members().take(limit) {
            output = output + &Crate::print_version(version, verbose);
        }
        let length = self.versions.len();
        if limit < length {
            output = output + &format!("\n... use -VV to show all {} versions\n", length);
        }
        output
    }

    pub fn print_keywords(&self, verbose: bool) -> String {
        if verbose {
            format!("{:#}", self.keywords)
        } else {
            format!("{}", self.keywords)
        }
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            format_args!(
                "{:<18}{} (https://crates.io/crates/{})",
                "Crate:", self.krate.name, self.krate.name
            ),
            format_args!("{:<18}{}", "Version:", self.krate.max_version),
            format_args!(
                "{:<18}{:?}",
                "Default features:", self.krate.default_features
            ),
            format_args!("{:<18}{:?}", "Features:", self.krate.features),
            format_args!("{:<18}{}", "Description:", self.krate.description),
            format_args!("{:<18}{}", "Downloads:", self.krate.downloads),
            format_args!("{:<18}{}", "Homepage:", self.krate.homepage),
            format_args!("{:<18}{}", "Documentation:", self.krate.documentation),
            format_args!("{:<18}{}", "Repository:", self.krate.repository),
            format_args!("{:<18}{}", "License:", self.krate.license),
            format_args!("{:<18}{:?}", "Keywords:", self.krate.keywords),
        )?;

        if f.alternate() {
            write!(
                f,
                "{}\n{}",
                format_args!(
                    "{:<18}{}  ({:#})",
                    "Created at:", self.krate.created_at, self.krate.created_at
                ),
                format_args!(
                    "{:<18}{}  ({:#})",
                    "Updated at:", self.krate.updated_at, self.krate.updated_at
                )
            )
        } else {
            let mut versions = String::new();
            for line in self.print_last_versions(5, false).lines() {
                versions += "\n";
                if !line.is_empty() {
                    versions = versions + "  " + line;
                }
            }

            write!(
                f,
                "{}\n{}\n",
                format_args!("{:<18}{:#}", "Last updated:", self.krate.updated_at),
                format_args!("{:<18}\n{}", "Version history:", versions)
            )
        }
    }
}
