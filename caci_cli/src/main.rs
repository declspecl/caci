use std::path::PathBuf;

use caci_core::CaciResult;
use caci_fs::{git::GitCaciFilesystemAgent, nop::NopCaciFilesystemAgent, CaciFilesystemAgent};
use clap::Parser;
use cli::{CaciCli, CaciCommands, CaciTemplate};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    match args.command {
        CaciCommands::Init { project_name, template } => {
            println!("INIT");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(GitCaciFilesystemAgent::new(PathBuf::from(project_name)))
                    }
                },
                None => {
                    Box::new(NopCaciFilesystemAgent::new(PathBuf::from(project_name)))
                }
            };

            caci_fs_agent.initalize_caci()?;
        },
        CaciCommands::New { project_name, template } => {
            println!("NEW");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(GitCaciFilesystemAgent::new(PathBuf::from(project_name)))
                    }
                },
                None => {
                    Box::new(NopCaciFilesystemAgent::new(PathBuf::from(project_name)))
                }
            };

            caci_fs_agent.initalize_caci()?;
        },
        CaciCommands::Run { } => {
            println!("run");
        }
    }

    return Ok(());
}