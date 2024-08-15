use std::{fs, path::PathBuf};

use caci_core::CaciResult;
use caci_fs::{config::{CaciConfig, CaciHook, CaciLocalHook, CaciRemoteHook}, git::GitCaciFilesystemAgent, native::NativeCaciFilesystemAgent, CaciFilesystemAgent};
use clap::Parser;
use cli::{CaciCli, CaciCommands};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    let mut caci_fs_agent: Box<dyn CaciFilesystemAgent> = match &args.command {
        CaciCommands::New { project_name, agent } => {
            let caci_config = match agent {
                cli::CaciVcsAgent::Git => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Git)
                },
                cli::CaciVcsAgent::Native => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Native)
                }
            };

            let repo_base_directory = PathBuf::from(project_name);

            match agent {
                cli::CaciVcsAgent::Git => Box::new(GitCaciFilesystemAgent::new(repo_base_directory, caci_config)),
                cli::CaciVcsAgent::Native => Box::new(NativeCaciFilesystemAgent::new(repo_base_directory, caci_config))
            }
        },
        CaciCommands::Init { agent } => {
            let caci_config = match agent {
                cli::CaciVcsAgent::Git => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Git)
                },
                cli::CaciVcsAgent::Native => {
                    CaciConfig::new(caci_fs::config::CaciVcsAgent::Native)
                }
            };

            let repo_base_directory = PathBuf::new();

            match agent {
                cli::CaciVcsAgent::Git => Box::new(GitCaciFilesystemAgent::new(repo_base_directory, caci_config)),
                cli::CaciVcsAgent::Native => Box::new(NativeCaciFilesystemAgent::new(repo_base_directory, caci_config))
            }
        },
        _ => {
            let caci_config = match CaciConfig::try_deserialize(&fs::read_to_string("caci.toml")?) {
                Ok(config) => config,
                Err(_) => CaciConfig::default()
            };

            match caci_config.vcs_agent {
                caci_fs::config::CaciVcsAgent::Git => {
                    let repo_base_directory = PathBuf::from(".");

                    Box::new(GitCaciFilesystemAgent::new(repo_base_directory, caci_config))
                },
                caci_fs::config::CaciVcsAgent::Native => {
                    let repo_base_directory = PathBuf::from(".");

                    Box::new(NativeCaciFilesystemAgent::new(repo_base_directory, caci_config))
                }
            }
        }
    };

    match args.command {
        CaciCommands::New { project_name: _, agent: _ } => {
            caci_fs_agent.initalize()?;
        },
        CaciCommands::Init { agent: _ } => {
            caci_fs_agent.initalize()?;
        },
        CaciCommands::Clean => {
            unimplemented!();
        },
        CaciCommands::Write => {
            unimplemented!();
        },
        CaciCommands::Hook(hook_command) => {
            match hook_command {
                cli::CaciHookCommands::Add (hook_add_command) => {
                    match hook_add_command {
                        cli::CaciHookAddCommands::Local { name, description, command, stage, output } => {
                            let stage = match stage {
                                cli::CaciHookStage::PreCommit => caci_fs::config::CaciHookStage::PreCommit,
                                cli::CaciHookStage::PrepareCommitMsg => caci_fs::config::CaciHookStage::PrepareCommitMsg,
                                cli::CaciHookStage::CommitMsg => caci_fs::config::CaciHookStage::CommitMsg,
                                cli::CaciHookStage::PostCommit => caci_fs::config::CaciHookStage::PostCommit,
                                cli::CaciHookStage::PrePush => caci_fs::config::CaciHookStage::PrePush
                            };

                            let output = match output {
                                cli::CaciHookOutput::Stdout => Some(caci_fs::config::CaciHookOutput::Stdout),
                                cli::CaciHookOutput::Commit => Some(caci_fs::config::CaciHookOutput::Commit),
                                cli::CaciHookOutput::Silent => Some(caci_fs::config::CaciHookOutput::Silent)
                            };

                            let new_hook = CaciLocalHook {
                                name,
                                description,
                                command,
                                stage,
                                output
                            };

                            caci_fs_agent.get_mut_caci_config().hooks.push(CaciHook::LocalHook(new_hook));
                        },
                        cli::CaciHookAddCommands::Remote { name, description, hook_url, hook_executor, stage, output } => {
                            let stage = match stage {
                                cli::CaciHookStage::PreCommit => caci_fs::config::CaciHookStage::PreCommit,
                                cli::CaciHookStage::PrepareCommitMsg => caci_fs::config::CaciHookStage::PrepareCommitMsg,
                                cli::CaciHookStage::CommitMsg => caci_fs::config::CaciHookStage::CommitMsg,
                                cli::CaciHookStage::PostCommit => caci_fs::config::CaciHookStage::PostCommit,
                                cli::CaciHookStage::PrePush => caci_fs::config::CaciHookStage::PrePush
                            };

                            let output = match output {
                                cli::CaciHookOutput::Stdout => Some(caci_fs::config::CaciHookOutput::Stdout),
                                cli::CaciHookOutput::Commit => Some(caci_fs::config::CaciHookOutput::Commit),
                                cli::CaciHookOutput::Silent => Some(caci_fs::config::CaciHookOutput::Silent)
                            };

                            let new_hook = CaciRemoteHook {
                                name,
                                description,
                                hook_url,
                                hook_executor,
                                stage,
                                output
                            };

                            caci_fs_agent.get_mut_caci_config().hooks.push(CaciHook::RemoteHook(new_hook));
                        }
                    }
                },
                cli::CaciHookCommands::Remove { name: _ } => {
                    unimplemented!();
                },
                cli::CaciHookCommands::Run { name: _ } => {
                    unimplemented!();
                }
            }
        }
    }

    caci_fs_agent.write_config()?;

    return Ok(());
}