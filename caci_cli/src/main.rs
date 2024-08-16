use std::{fs, path::PathBuf};

use caci_core::{
    model::{CaciConfig, Hook, HookOutput, HookStage, LocalHook, RemoteHook, VcsAgent},
    CaciResult
};
use caci_fs::{
    git::GitFilesystemController, native::NativeFilesystemController, FilesystemController
};
use clap::Parser;
use cli::{CaciCli, CliCommands, CliHookAddCommands, CliHookCommands, CliVcsAgent};

pub mod cli;

fn main() -> CaciResult<()> {
    let args = CaciCli::parse();

    let mut caci_fs_agent: Box<dyn FilesystemController> = match &args.command {
        CliCommands::New {
            project_name,
            agent
        } => {
            let caci_config = CaciConfig::new(agent.clone().into());

            let repo_base_directory = PathBuf::from(project_name);

            match agent {
                CliVcsAgent::Git => Box::new(GitFilesystemController::new(
                    repo_base_directory,
                    caci_config
                )),
                CliVcsAgent::Native => Box::new(NativeFilesystemController::new(
                    repo_base_directory,
                    caci_config
                ))
            }
        },
        CliCommands::Init { agent } => {
            let caci_config = CaciConfig::new(agent.clone().into());
            let repo_base_directory = PathBuf::new();

            match agent {
                CliVcsAgent::Git => Box::new(GitFilesystemController::new(
                    repo_base_directory,
                    caci_config
                )),
                CliVcsAgent::Native => Box::new(NativeFilesystemController::new(
                    repo_base_directory,
                    caci_config
                ))
            }
        },
        _ => {
            let caci_config = match CaciConfig::try_deserialize(&fs::read_to_string("caci.toml")?) {
                Ok(config) => config,
                Err(_) => CaciConfig::default()
            };

            match caci_config.vcs_agent {
                VcsAgent::Git => {
                    let repo_base_directory = PathBuf::from(".");

                    Box::new(GitFilesystemController::new(
                        repo_base_directory,
                        caci_config
                    ))
                },
                VcsAgent::Native => {
                    let repo_base_directory = PathBuf::from(".");

                    Box::new(NativeFilesystemController::new(
                        repo_base_directory,
                        caci_config
                    ))
                }
            }
        }
    };

    match args.command {
        CliCommands::New {
            project_name: _,
            agent: _
        } => {
            caci_fs_agent.initalize()?;
        },
        CliCommands::Init { agent: _ } => {
            caci_fs_agent.initalize()?;
        },
        CliCommands::Clean => {
            caci_fs_agent.clean_hooks()?;
            caci_fs_agent.clean_scripts()?;
        },
        CliCommands::Write => {
            unimplemented!();
        },
        CliCommands::Hook(hook_command) => match hook_command {
            CliHookCommands::Add(hook_add_command) => match hook_add_command {
                CliHookAddCommands::Local {
                    name,
                    description,
                    command,
                    stage,
                    output
                } => {
                    let stage: HookStage = stage.into();
                    let output: HookOutput = output.into();

                    let new_hook = LocalHook::new(name, description, command, stage, output);

                    caci_fs_agent
                        .get_mut_config()
                        .hooks
                        .push(Hook::LocalHook(new_hook));
                },
                CliHookAddCommands::Remote {
                    name,
                    description,
                    hook_url,
                    hook_executor,
                    stage,
                    output
                } => {
                    let stage: HookStage = stage.into();
                    let output: HookOutput = output.into();

                    let new_hook =
                        RemoteHook::new(name, description, hook_url, hook_executor, stage, output);

                    caci_fs_agent
                        .get_mut_config()
                        .hooks
                        .push(Hook::RemoteHook(new_hook));
                }
            },
            CliHookCommands::Remove { name: _ } => {
                unimplemented!();
            },
            CliHookCommands::Run { name: _ } => {
                unimplemented!();
            }
        }
    }

    caci_fs_agent.write_config()?;

    return Ok(());
}
