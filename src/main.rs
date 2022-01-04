use action_cargo_deny::cli::Opt;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let opt = Opt::parse();
    action_cargo_deny::print_opt_info(&opt);
    action_cargo_deny::run(opt)
}
