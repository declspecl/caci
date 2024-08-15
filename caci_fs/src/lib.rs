pub mod native;
pub mod git;
pub mod config;

use std::{fs, path::{Path, PathBuf}};

use caci_core::CaciResult;
use config::CaciConfig;

pub trait CaciFilesystemAgent {
    // (expected) implementer members getters
    fn get_caci_config(&self) -> &CaciConfig;
    fn get_mut_caci_config(&mut self) -> &mut CaciConfig;
    fn get_repo_base_directory(&self) -> &Path;
    fn get_repo_agent_directory(&self) -> &Path;

    // implementer misc getters
    fn get_repo_agent_hooks_directory(&self) -> PathBuf;

    // implementer filesystem operations
    fn write_hooks(&self) -> CaciResult<()>;
    fn initalize(&self) -> CaciResult<()>;

    fn write_config(&self) -> CaciResult<()> {
        fs::write(&self.get_repo_base_directory().join("caci.toml"), self.get_caci_config().try_serialize()?.as_bytes())?;

        return Ok(());
    }
}