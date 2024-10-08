use std::{fs, process::Command};

use error::{CaciError, CaciResult};
use config::{CaciConfig, Hook, HookOutput, HookStage, LocalHook, RemoteHook, VcsAgent};
use agent::{git::GitFilesystemController, native::NativeFilesystemController, FilesystemController};
use clap::Parser;
use cli::{CaciCli, CliCommands, CliHookAddCommands, CliHookCommands, CliVcsAgent};

pub mod cli;
pub mod error;
pub mod agent;
pub mod config;

fn main() -> CaciResult<()> {
    let cwd = std::env::current_dir()?;

    let args = CaciCli::parse();

    let mut caci_fs_controller: Box<dyn FilesystemController> = match &args.command {
        CliCommands::New { project_name, agent } => {
            let caci_config = CaciConfig::from_vcs_agent(agent.clone().into());

            let repo_base_directory = cwd.join(project_name);

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
            let caci_config = CaciConfig::from_vcs_agent(agent.clone().into());
            let repo_base_directory = cwd;

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
            let caci_config = CaciConfig::try_deserialize(&fs::read_to_string("caci.toml")?)?;

            let repo_base_directory = cwd;

            match caci_config.vcs_agent {
                VcsAgent::Git => Box::new(GitFilesystemController::new(
                    repo_base_directory,
                    caci_config
                )),
                VcsAgent::Native => Box::new(NativeFilesystemController::new(
                    repo_base_directory,
                    caci_config
                ))
            }
        }
    };

    match args.command {
        CliCommands::New {
            project_name: _,
            agent: _
        } => {
            caci_fs_controller.initalize_all()?;
        },
        CliCommands::Init { agent: _ } => {
            caci_fs_controller.initalize_all()?;
        },
        CliCommands::Clean => {
            caci_fs_controller.clean_hooks()?;
            // TODO: look into if this is a good feature
            // caci_fs_controller.clean_remote_scripts()?;
        },
        CliCommands::Write => {
            caci_fs_controller.write_config()?;
            caci_fs_controller.write_hooks()?;
        },
        CliCommands::Hook(hook_command) => match hook_command {
            CliHookCommands::Add(hook_add_command) => match hook_add_command {
                CliHookAddCommands::Local {
                    name,
                    description,
                    script_filename,
                    executor,
                    stage,
                    output
                } => {
                    let stage: HookStage = stage.into();
                    let output: HookOutput = output.into();

                    let new_hook = LocalHook::new(
                        name,
                        description,
                        script_filename,
                        executor,
                        stage,
                        output
                    );

                    caci_fs_controller
                        .get_mut_config()
                        .hooks
                        .push(Hook::LocalHook(new_hook));

                    caci_fs_controller.write_config()?;
                    caci_fs_controller.write_hooks()?;
                },
                CliHookAddCommands::Remote {
                    name,
                    description,
                    script_url,
                    script_filename,
                    executor,
                    stage,
                    output
                } => {
                    let stage: HookStage = stage.into();
                    let output: HookOutput = output.into();

                    let new_hook = RemoteHook::new(
                        name,
                        description,
                        script_url,
                        script_filename,
                        executor,
                        stage,
                        output
                    );

                    caci_fs_controller
                        .get_mut_config()
                        .hooks
                        .push(Hook::RemoteHook(new_hook));

                    caci_fs_controller.write_config()?;
                    caci_fs_controller.write_hooks()?;
                }
            },
            CliHookCommands::Remove { name: _ } => {
                unimplemented!();
            },
            CliHookCommands::Run { stage } => {
                let stage = stage.into();
                let hooks_by_stage = caci_fs_controller.get_config().get_hooks_by_stage();

                let target_hooks = hooks_by_stage.get(&stage).ok_or(CaciError::UnknownHookStage(
                    stage.to_vcs_stage_name()
                ))?;

                for hook in target_hooks.into_iter() {
                    let executor = match hook {
                        Hook::LocalHook(local_hook) => local_hook.executor.clone(),
                        Hook::RemoteHook(remote_hook) => remote_hook.executor.clone()
                    };

                    let hook_script_command = match hook {
                        Hook::LocalHook(local_hook) => caci_fs_controller
                            .caci_scripts_directory()
                            .join(local_hook.script_filename.as_str()),
                        Hook::RemoteHook(remote_hook) => caci_fs_controller
                            .caci_scripts_directory()
                            .join(remote_hook.script_filename.as_str())
                    };

                    println!(
                        "Running hook: {}",
                        hook_script_command.to_string_lossy()
                    );

                    let mut child = Command::new(executor).arg(hook_script_command).spawn()?;

                    child.wait()?;
                }
            }
        }
    }

    return Ok(());
}
