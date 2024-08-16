pub mod git;
pub mod native;

use std::{fs, path::PathBuf};

use caci_core::{model::CaciConfig, CaciResult};

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

    fn write_scripts(&self) -> CaciResult<()>;
    fn write_hooks(&self) -> CaciResult<()>;
    fn write_config(&self) -> CaciResult<()> {
        fs::write(
            self.repo_base_directory().join("caci.toml").as_path(),
            self.get_config().try_serialize()?.as_bytes()
        )?;

        return Ok(());
    }
    fn initialize_vcs(&self) -> CaciResult<()>;
    fn initalize(&self) -> CaciResult<()> {
        self.initialize_vcs()?;
        self.write_config()?;
        self.write_hooks()?;
        self.write_scripts()?;

        return Ok(());
    }

    fn clean_hooks(&self) -> CaciResult<()> {
        fs::remove_dir_all(self.repo_vcs_hooks_directory().as_path())?;
        fs::create_dir_all(self.repo_vcs_hooks_directory().as_path())?;

        return Ok(());
    }
    fn clean_scripts(&self) -> CaciResult<()> {
        fs::remove_dir_all(self.caci_scripts_directory().as_path())?;
        fs::create_dir_all(self.caci_scripts_directory().as_path())?;

        return Ok(());
    }
}
