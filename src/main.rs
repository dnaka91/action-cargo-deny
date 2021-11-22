use action_cargo_deny::cli::Opt;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    action_cargo_deny::run(Opt::from_args())
}
