use std::{fs, path::PathBuf};

use crate::{config::CaciConfig, error::CaciResult, FilesystemController};

pub struct NativeFilesystemController {
    repo_base_directory: PathBuf,
    config: CaciConfig
}

impl NativeFilesystemController {
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

impl FilesystemController for NativeFilesystemController {
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
        return self.caci_directory();
    }

    fn repo_vcs_hooks_directory(&self) -> PathBuf {
        return self.caci_directory().join("hooks");
    }

    fn initialize_vcs(&self) -> CaciResult<()> {
        fs::create_dir_all(self.repo_vcs_directory().as_path())?;

        return Ok(());
    }
}
