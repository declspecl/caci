use std::path::PathBuf;

use caci_core::CaciResult;
use caci_fs::{git::GitCaciFilesystemAgent, nop::NopCaciFilesystemAgent, CaciFilesystemAgent};
use clap::Parser;
use cli::{CaciCli, CaciCommands, CaciTemplate};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    match args.command {
        CaciCommands::Init(init_args) => {
            println!("INIT");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match init_args.template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(GitCaciFilesystemAgent::new(PathBuf::from(init_args.project_name)))
                    }
                },
                None => {
                    Box::new(NopCaciFilesystemAgent::new(PathBuf::from(init_args.project_name)))
                }
            };

            caci_fs_agent.initalize_caci()?;
        },
        CaciCommands::New(new_args) => {
            println!("NEW");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match new_args.template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(GitCaciFilesystemAgent::new(PathBuf::from(new_args.project_name)))
                    }
                },
                None => {
                    Box::new(NopCaciFilesystemAgent::new(PathBuf::from(new_args.project_name)))
                }
            };

            caci_fs_agent.initalize_caci()?;
        },
        CaciCommands::Run(_run_args) => {
            println!("run");
        }
    }

    return Ok(());
}