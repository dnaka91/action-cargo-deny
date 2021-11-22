use action_cargo_deny::cli::Opt;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    action_cargo_deny::run(opt.deny, opt.level)
}
