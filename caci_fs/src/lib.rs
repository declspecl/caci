pub mod git;

use std::path::PathBuf;

pub trait CaciFilesystemManager {
    fn get_repo_hooks_directory(&self) -> PathBuf;
}