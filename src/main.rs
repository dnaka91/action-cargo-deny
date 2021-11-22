use action_cargo_deny::cli::Opt;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    action_cargo_deny::print_opt_info(&opt);
    action_cargo_deny::run(opt)
}
