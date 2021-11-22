use std::{io::BufRead, process::Command};

use anyhow::{bail, Context, Result};
use cli::Opt;
use log::LogEntry;
use strum::{Display, EnumString, EnumVariantNames};

pub mod cli;
pub mod github;
pub mod log;

pub fn print_opt_info(opt: &Opt) {
    println!("Will report {}s or worse", opt.report_level);
    println!("Will fail on {}s or worse", opt.fail_level);
    println!(
        "Checks to perform: {}",
        opt.cargo_deny
            .checks
            .iter()
            .fold(String::new(), |mut buf, check| {
                if !buf.is_empty() {
                    buf.push_str(", ");
                }
                buf.push_str(check.as_ref());
                buf
            })
    )
}

pub fn run(opt: Opt) -> Result<()> {
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

    if let Some(path) = opt.cargo_deny.manifest_path {
        cmd.arg("--manifest-path");
        cmd.arg(path);
    }

    cmd.arg("check");

    for check in opt.cargo_deny.checks {
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

    let mut found_level = None;

    for line in output.stderr.lines() {
        let entry = serde_json::from_str::<LogEntry>(&line?)?;
        let values = entry.print();

        found_level = Some(found_level.unwrap_or(values.level).min(values.level));

        if opt.report_level < values.level {
            continue;
        }

        let log_fn = match values.level {
            PrintLevel::Error => github::error,
            PrintLevel::Warning => github::warning,
            PrintLevel::Notice => github::notice,
        };

        log_fn(values.title.as_deref(), &values.message);
    }

    if let Some(found_level) = found_level {
        if opt.fail_level >= found_level {
            bail!(
                "failed due to finding one or more {}s or worse",
                opt.fail_level
            );
        }
    }

    Ok(())
}

pub struct PrintValues {
    title: Option<String>,
    message: String,
    level: PrintLevel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumVariantNames, Display)]
#[strum(serialize_all = "lowercase")]
pub enum PrintLevel {
    Error,
    Warning,
    Notice,
}
