pub mod nop;
pub mod git;

use std::path::{Path, PathBuf};

use caci_core::CaciResult;

pub trait CaciFilesystemAgent {
    fn get_repo_base_directory(&self) -> &Path;

    fn get_repo_caci_directory(&self) -> PathBuf {
        return self.get_repo_base_directory().join(".caci");
    }

    fn get_repo_vcs_directory(&self) -> PathBuf;
    fn get_repo_vcs_hooks_directory(&self) -> PathBuf;

    fn initalize_caci(&self) -> CaciResult<()>;
}