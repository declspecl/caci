use caci_core::CaciResult;
use clap::Parser;
use cli::{CaciCli, CaciCommands};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    match args.command {
        CaciCommands::Init => {
            println!("init");
        },
        CaciCommands::New(new_args) => {
            println!("new project: {}", new_args.project_name);
        },
        CaciCommands::Run => {
            println!("run");
        }
    }

    return Ok(());
}