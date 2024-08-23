use std::{fs, path::PathBuf, process::Command};

use caci_core::{model::CaciConfig, CaciResult};

use crate::FilesystemController;

pub struct GitFilesystemController {
    repo_base_directory: PathBuf,
    config: CaciConfig
}

impl GitFilesystemController {
    pub fn new(
        repo_base_directory: PathBuf,
        config: CaciConfig
    ) -> Self {
        return Self {
            repo_base_directory,
            config
        };
    }
}

impl FilesystemController for GitFilesystemController {
    fn get_config(&self) -> &CaciConfig {
        return &self.config;
    }

    fn get_mut_config(&mut self) -> &mut CaciConfig {
        return &mut self.config;
    }

    fn repo_base_directory(&self) -> PathBuf {
        return self.repo_base_directory.clone();
    }

    fn repo_vcs_directory(&self) -> PathBuf {
        return self.repo_base_directory().join(".git");
    }

    fn repo_vcs_hooks_directory(&self) -> PathBuf {
        return self.repo_vcs_directory().join("hooks");
    }

    fn write_config(&self) -> CaciResult<()> {
        fs::write(
            self.repo_base_directory().join("caci.toml").as_path(),
            self.get_config().try_serialize()?.as_bytes()
        )?;

        return Ok(());
    }
    fn initialize_vcs(&self) -> CaciResult<()> {
        Command::new("git")
            .arg("init")
            .arg(self.repo_base_directory().as_path())
            .output()?;

        return Ok(());
    }
}
