use std::{fs, path::{Path, PathBuf}};

use caci_core::CaciResult;

use crate::CaciFilesystemAgent;

pub struct GitCaciFilesystemAgent {
    repo_base_directory: PathBuf
}

impl GitCaciFilesystemAgent {
    pub fn new(repo_base_directory: PathBuf) -> Self {
        return Self { repo_base_directory };
    }
}

impl CaciFilesystemAgent for GitCaciFilesystemAgent {
    fn get_repo_base_directory(&self) -> &Path {
        return &self.repo_base_directory;
    }

    fn get_repo_vcs_directory(&self) -> PathBuf {
        return self.repo_base_directory.join(".git");
    }

    fn get_repo_vcs_hooks_directory(&self) -> PathBuf {
        return self.get_repo_vcs_directory().join("hooks");
    }

    fn initalize_caci(&self) -> CaciResult<()> {
        let caci_directory = self.get_repo_caci_directory();

        fs::create_dir_all(&caci_directory)?;
        fs::write(&caci_directory.join("caci.toml"), "TODO")?;

        return Ok(());
    }
}