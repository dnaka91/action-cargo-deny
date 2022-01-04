use std::path::PathBuf;

use clap::{ArgEnum, Args, Parser};
use strum::AsRefStr;

use crate::PrintLevel;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Opt {
    #[clap(flatten)]
    pub cargo_deny: CargoDenyOpt,
    #[clap(long, arg_enum, default_value_t = PrintLevel::Warning)]
    pub report_level: PrintLevel,
    #[clap(long, arg_enum, default_value_t = PrintLevel::Error)]
    pub fail_level: PrintLevel,
}

#[derive(Args)]
pub struct CargoDenyOpt {
    /// The path of a Cargo.toml to use as the context for the operation.
    #[clap(long)]
    pub manifest_path: Option<PathBuf>,
    /// The check(s) to perform.
    #[clap(arg_enum)]
    pub checks: Vec<Check>,
}

#[derive(Clone, Copy, ArgEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Check {
    Advisories,
    Bans,
    Licenses,
    Sources,
}
