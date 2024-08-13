use std::path::PathBuf;

use crate::CaciFilesystemManager;

pub struct GitCaciFilesystemImpl {

}

impl CaciFilesystemManager for GitCaciFilesystemImpl {
    fn get_repo_hooks_directory(&self) -> PathBuf {
        return PathBuf::from(".git/hooks");
    }
}