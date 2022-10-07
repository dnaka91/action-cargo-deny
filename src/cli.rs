use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};
use strum::AsRefStr;

use crate::PrintLevel;

#[derive(Parser)]
#[command(about, author, version)]
pub struct Opt {
    #[command(flatten)]
    pub cargo_deny: CargoDenyOpt,
    #[arg(long, value_enum, default_value_t = PrintLevel::Warning)]
    pub report_level: PrintLevel,
    #[arg(long, value_enum, default_value_t = PrintLevel::Error)]
    pub fail_level: PrintLevel,
}

#[derive(Args)]
pub struct CargoDenyOpt {
    /// The path of a Cargo.toml to use as the context for the operation.
    #[arg(long)]
    pub manifest_path: Option<PathBuf>,
    /// The check(s) to perform.
    #[arg(value_enum)]
    pub checks: Vec<Check>,
}

#[derive(Clone, Copy, AsRefStr, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Check {
    Advisories,
    Bans,
    Licenses,
    Sources,
}
