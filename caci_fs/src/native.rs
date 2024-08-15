use std::{fs, path::{Path, PathBuf}};

use caci_core::CaciResult;

use crate::{config::CaciConfig, CaciFilesystemAgent};

pub struct NativeCaciFilesystemAgent {
    repo_base_directory: PathBuf,
    repo_agent_directory: PathBuf,
    caci_config: CaciConfig
}

impl NativeCaciFilesystemAgent {
    pub fn new(
        repo_base_directory: PathBuf,
        caci_config: CaciConfig
    ) -> Self
    {
        let repo_agent_directory = repo_base_directory.join(".caci");

        return Self { repo_base_directory, repo_agent_directory, caci_config };
    }
}

impl CaciFilesystemAgent for NativeCaciFilesystemAgent {
    fn get_caci_config(&self) -> &CaciConfig {
        return &self.caci_config;
    }

    fn get_repo_base_directory(&self) -> &Path {
        return &self.repo_base_directory;
    }

    fn get_repo_agent_directory(&self) -> &Path {
        return &self.repo_agent_directory;
    }

    fn get_repo_agent_hooks_directory(&self) -> PathBuf {
        return self.repo_agent_directory.join("hooks");
    }

    fn write_hooks(&self) -> CaciResult<()> {
        unimplemented!();
    }

    fn initalize_caci(&self) -> CaciResult<()> {
        fs::write(&self.get_repo_base_directory().join("caci.toml"), self.get_caci_config().try_serialize()?.as_bytes())?;

        return Ok(());
    }
}