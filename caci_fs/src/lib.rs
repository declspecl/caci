pub mod git;
pub mod native;

use std::{collections::HashMap, fs, path::PathBuf};

use caci_core::{
    model::{CaciConfig, Hook, HookStage},
    CaciResult
};

pub trait FilesystemController {
    fn get_config(&self) -> &CaciConfig;
    fn get_mut_config(&mut self) -> &mut CaciConfig;

    fn repo_base_directory(&self) -> PathBuf;
    fn repo_vcs_directory(&self) -> PathBuf;
    fn repo_vcs_hooks_directory(&self) -> PathBuf;
    fn caci_directory(&self) -> PathBuf {
        return self.repo_base_directory().join(".caci");
    }
    fn caci_scripts_directory(&self) -> PathBuf {
        return self.caci_directory().join("scripts");
    }

    fn write_hooks(&self) -> CaciResult<()> {
        let hooks_by_stage = self.get_config().hooks.iter().fold(
            vec![
                HookStage::PreCommit,
                HookStage::PrepareCommitMsg,
                HookStage::CommitMsg,
                HookStage::PostCommit,
                HookStage::PrePush,
            ]
            .into_iter()
            .map(|stage| (stage, Vec::new()))
            .collect::<HashMap<HookStage, Vec<Hook>>>(),
            |mut acc, hook| {
                let hook_stage = match hook {
                    Hook::LocalHook(local_hook) => local_hook.stage,
                    Hook::RemoteHook(remote_hook) => remote_hook.stage
                };

                acc.get_mut(&hook_stage).unwrap().push(hook.to_owned());

                return acc;
            }
        );

        for (stage, hooks) in hooks_by_stage.iter() {
            let hook_content = hooks
                .iter()
                .map(|hook| {
                    let executor = match hook {
                        Hook::LocalHook(local_hook) => local_hook.executor.to_owned(),
                        Hook::RemoteHook(remote_hook) => remote_hook.executor.to_owned()
                    };

                    // TODO: switch to use defined script paths option. throw error if script DNE in any of the paths
                    let hook_script_command = match hook {
                        Hook::LocalHook(local_hook) => self.caci_scripts_directory().join(local_hook.name.as_str()),
                        Hook::RemoteHook(remote_hook) => self.caci_scripts_directory().join(remote_hook.name.as_str())
                    }
                    .to_string_lossy()
                    .to_string();

                    return format!("{} {}", executor, hook_script_command);
                })
                .collect::<Vec<String>>()
                .join("\n");

            println!(
                "Writing hook content: {} to {}",
                hook_content,
                self.repo_vcs_hooks_directory()
                    .join(stage.to_vcs_stage_name())
                    .to_string_lossy()
                    .to_string()
            );

            fs::write(
                self.repo_vcs_hooks_directory()
                    .join(stage.to_vcs_stage_name())
                    .as_path(),
                hook_content.as_bytes()
            )?;
        }

        return Ok(());
    }

    fn write_config(&self) -> CaciResult<()> {
        fs::write(
            self.repo_base_directory().join("caci.toml").as_path(),
            self.get_config().try_serialize()?.as_bytes()
        )?;

        return Ok(());
    }
    fn initialize_caci(&self) -> CaciResult<()> {
        fs::create_dir_all(self.caci_directory().as_path())?;
        fs::create_dir_all(self.caci_scripts_directory().as_path())?;

        return Ok(());
    }
    fn initialize_vcs(&self) -> CaciResult<()>;
    fn initalize_all(&self) -> CaciResult<()> {
        self.initialize_caci()?;
        self.initialize_vcs()?;
        self.write_config()?;
        self.download_remote_hooks()?;
        self.write_hooks()?;

        return Ok(());
    }

    fn clean_hooks(&self) -> CaciResult<()> {
        fs::remove_dir_all(self.repo_vcs_hooks_directory().as_path())?;
        fs::create_dir_all(self.repo_vcs_hooks_directory().as_path())?;

        return Ok(());
    }

    fn download_remote_hooks(&self) -> CaciResult<()> {
        for hook in self.get_config().hooks.iter() {
            if let Hook::RemoteHook(remote_hook) = hook {
                let hook_script_content = reqwest::blocking::get(remote_hook.script_url.as_str())?.text()?;

                fs::write(
                    self.repo_vcs_hooks_directory()
                        .join(remote_hook.name.as_str())
                        .as_path(),
                    hook_script_content.as_bytes()
                )?;
            }
        }

        return Ok(());
    }
}
