use std::path::PathBuf;

use caci_core::CaciResult;
use caci_fs::{config::CaciConfig, git::GitCaciFilesystemAgent, native::NativeCaciFilesystemAgent, CaciFilesystemAgent};
use clap::Parser;
use cli::{CaciCli, CaciCommands};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match args.command {
        CaciCommands::New { project_name, agent } => {
            let project_name = PathBuf::from(project_name);
            let caci_config = CaciConfig::new(match agent {
                cli::CaciVcsAgent::Git => caci_fs::config::CaciVcsAgent::Git,
                cli::CaciVcsAgent::Native => caci_fs::config::CaciVcsAgent::Native
            });

            match agent {
                CaciVcsAgent::Git => {
                    Box::new(GitCaciFilesystemAgent::new(project_name));
                },
                CaciVcsAgent::Native => {
                    Box::new(NativeCaciFilesystemAgent::new(PathBuf::from(project_name)))
                }
            }
        },
        CaciCommands::Init { project_name, agent } => {
            match agent {
                CaciVcsAgent::Git => {
                    Box::new(caci_fs::git::GitCaciFilesystemAgent::new(PathBuf::from(project_name)))
                },
                CaciVcsAgent::Native => {
                    Box::new(NativeCaciFilesystemAgent::new(PathBuf::from(project_name)))
                }
            }
        },
    }

    match args.command {
        CaciCommands::Init { project_name, agent } => {
            println!("INIT");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(caci_fs::git::GitCaciFilesystemAgent::new(PathBuf::from(project_name)))
                    }
                },
                None => {
                    Box::new(NativeCaciFilesystemAgent::new(PathBuf::from(project_name)))
                }
            };
            
            caci_fs_agent.initalize_caci()?;
        },
        CaciCommands::New { project_name, template } => {
            println!("NEW");

            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = match template {
                Some(template) => {
                    match template {
                        CaciTemplate::Git => Box::new(caci_fs::git::GitCaciFilesystemAgent::new(PathBuf::from(project_name)))
                    }
                },
                None => {
                    Box::new(NativeCaciFilesystemAgent::new(PathBuf::from(project_name)))
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