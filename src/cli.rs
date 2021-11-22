use std::path::PathBuf;

use structopt::{clap::AppSettings, StructOpt};
use strum::{AsRefStr, EnumString, EnumVariantNames, VariantNames};

use crate::PrintLevel;

#[derive(StructOpt)]
#[structopt(about, author, global_setting = AppSettings::ColoredHelp)]
pub struct Opt {
    #[structopt(flatten)]
    pub deny: DenyOpt,
    #[structopt(long, default_value = "warning", possible_values = PrintLevel::VARIANTS)]
    pub level: PrintLevel,
}

#[derive(StructOpt)]
pub struct DenyOpt {
    /// The path of a Cargo.toml to use as the context for the operation.
    #[structopt(long)]
    pub manifest_path: Option<PathBuf>,
    /// The check(s) to perform.
    #[structopt(possible_values = Check::VARIANTS)]
    pub checks: Vec<Check>,
}

#[derive(Clone, Copy, EnumString, EnumVariantNames, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Check {
    Advisories,
    Bans,
    Licenses,
    Sources,
}
