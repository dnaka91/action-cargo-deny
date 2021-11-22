use std::{io::BufRead, process::Command};

use anyhow::{bail, Context, Result};
use cli::DenyOpt;
use log::LogEntry;
use strum::{EnumString, EnumVariantNames};

pub mod cli;
pub mod github;
pub mod log;

pub fn run(deny_opt: DenyOpt, max_level: PrintLevel) -> Result<()> {
    let path = which::which("cargo-deny").context("failed finding `cargo-deny` binary")?;
    let mut cmd = Command::new(path);
    cmd.args([
        "--workspace",
        "--all-features",
        "--format",
        "json",
        "--log-level",
        "trace",
    ]);

    if let Some(path) = deny_opt.manifest_path {
        cmd.arg("--manifest-path");
        cmd.arg(path);
    }

    cmd.arg("check");

    for check in deny_opt.checks {
        cmd.arg(check.as_ref());
    }

    let output = cmd.output().context("failed running `cargo-deny`")?;
    let status = output.status.code().unwrap_or_default();

    if status > 1 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "failed running cargo-deny, exited with status code {}\n\n{}",
            status,
            stderr
        );
    }

    for line in output.stderr.lines() {
        let entry = serde_json::from_str::<LogEntry>(&line?)?;
        let values = entry.print();

        if max_level < values.level {
            continue;
        }

        let log_fn = match values.level {
            PrintLevel::Error => github::error,
            PrintLevel::Warning => github::warning,
            PrintLevel::Notice => github::notice,
        };

        log_fn(values.title.as_deref(), &values.message);
    }

    Ok(())
}

pub struct PrintValues {
    title: Option<String>,
    message: String,
    level: PrintLevel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PrintLevel {
    Error,
    Warning,
    Notice,
}
