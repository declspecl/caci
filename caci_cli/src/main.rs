use std::{fs, path::PathBuf};

use caci_core::CaciResult;
use caci_fs::{config::CaciConfig, git::GitCaciFilesystemAgent, native::NativeCaciFilesystemAgent, CaciFilesystemAgent};
use clap::Parser;
use cli::{CaciCli, CaciCommands};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    let caci_config = match args.command {
        CaciCommands::New { project_name: _, ref agent } => {
            match agent {
                cli::CaciVcsAgent::Git => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Git)
                },
                cli::CaciVcsAgent::Native => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Native)
                }
            }
        },
        CaciCommands::Init { ref agent } => {
            match agent {
                cli::CaciVcsAgent::Git => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Git)
                },
                cli::CaciVcsAgent::Native => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Native)
                }
            }
        },
        _ => {
            match CaciConfig::try_deserialize(&fs::read_to_string("caci.toml")?) {
                Ok(config) => config,
                Err(_) => CaciConfig::default()
            }
        }
    };

    match args.command {
        CaciCommands::New { project_name, agent } => {
            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = {
                let repo_base_directory = PathBuf::from(project_name);

                match agent {
                    cli::CaciVcsAgent::Git => Box::new(GitCaciFilesystemAgent::new(repo_base_directory, caci_config)),
                    cli::CaciVcsAgent::Native => Box::new(NativeCaciFilesystemAgent::new(repo_base_directory, caci_config))
                }
            };
        },
        CaciCommands::Init { agent } => {
            let caci_fs_agent: Box<dyn CaciFilesystemAgent> = {
                let repo_base_directory = PathBuf::new();

                match agent {
                    cli::CaciVcsAgent::Git => Box::new(GitCaciFilesystemAgent::new(repo_base_directory, caci_config)),
                    cli::CaciVcsAgent::Native => Box::new(NativeCaciFilesystemAgent::new(repo_base_directory, caci_config))
                }
            };
        },
        CaciCommands::Clean => {
            unimplemented!();
        },
        CaciCommands::Write => {
            unimplemented!();
        },
        CaciCommands::Hook(hook_command) => {
            match hook_command {
                cli::CaciHookCommands::Add { name, command } => {
                    unimplemented!();
                },
                cli::CaciHookCommands::Remove { name } => {
                    unimplemented!();
                }
            }
        }
    }

    return Ok(());
}