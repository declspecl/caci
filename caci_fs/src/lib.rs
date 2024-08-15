pub mod native;
pub mod git;
pub mod config;

use std::path::{Path, PathBuf};

use caci_core::CaciResult;
use config::CaciConfig;

pub trait CaciFilesystemAgent {
    // (expected) implementer members getters
    fn get_caci_config(&self) -> &CaciConfig;
    fn get_repo_base_directory(&self) -> &Path;
    fn get_repo_agent_directory(&self) -> &Path;

    // implementer misc getters
    fn get_repo_agent_hooks_directory(&self) -> PathBuf;

    // implementer filesystem operations
    fn write_hooks(&self) -> CaciResult<()>;
    fn initalize_caci(&self) -> CaciResult<()>;
}