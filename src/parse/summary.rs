use std::fmt::{self, Display};

use serde::Deserialize;

use crate::{PrintLevel, PrintValues};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    #[serde(default)]
    advisories: Stats,
    #[serde(default)]
    bans: Stats,
    #[serde(default)]
    licenses: Stats,
    #[serde(default)]
    sources: Stats,
}

impl Summary {
    pub fn print(&self) -> PrintValues {
        PrintValues {
            title: None,
            message: format!(
                "Statistics:\nadvisories {}\nbans       {}\nlicenses   {}\nsources    {}",
                self.advisories, self.bans, self.licenses, self.sources,
            ),
            level: PrintLevel::Notice,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stats {
    errors: u32,
    warnings: u32,
    notes: u32,
    helps: u32,
}

impl Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>6}: {} errors, {} warnings, {} notes",
            if self.errors > 0 { "FAILED" } else { "ok" },
            self.errors,
            self.warnings,
            self.notes + self.helps
        )
    }
}
