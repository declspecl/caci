use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::CaciResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VcsAgent {
    Native,
    Git
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookOutput {
    Stdout,
    CommitMsg,
    Silent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookStage {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PrePush
}

impl HookStage {
    pub fn to_vcs_stage_name(&self) -> String {
        return match self {
            HookStage::PreCommit => "pre-commit".to_string(),
            HookStage::PrepareCommitMsg => "prepare-commit-msg".to_string(),
            HookStage::CommitMsg => "commit-msg".to_string(),
            HookStage::PostCommit => "post-commit".to_string(),
            HookStage::PrePush => "pre-push".to_string()
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalHook {
    pub name: String,
    pub description: Option<String>,
    pub script_filename: String,
    pub executor: String,
    pub stage: HookStage,
    pub output: HookOutput
}

impl LocalHook {
    pub fn new(
        name: String,
        description: Option<String>,
        script_filename: String,
        executor: String,
        stage: HookStage,
        output: HookOutput
    ) -> LocalHook {
        return LocalHook {
            name,
            description,
            script_filename,
            executor,
            stage,
            output
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RemoteHook {
    pub name: String,
    pub description: Option<String>,
    pub script_url: String,
    pub script_filename: String,
    pub executor: String,
    pub stage: HookStage,
    pub output: HookOutput
}

impl RemoteHook {
    pub fn new(
        name: String,
        description: Option<String>,
        script_url: String,
        script_filename: String,
        executor: String,
        stage: HookStage,
        output: HookOutput
    ) -> RemoteHook {
        return RemoteHook {
            name,
            description,
            script_url,
            script_filename,
            executor,
            stage,
            output
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Hook {
    LocalHook(LocalHook),
    RemoteHook(RemoteHook)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CaciConfig {
    pub vcs_agent: VcsAgent,
    pub script_paths: Vec<PathBuf>,
    pub hooks: Vec<Hook>
}

impl CaciConfig {
    pub fn new(
        vcs_agent: VcsAgent,
        script_paths: Vec<PathBuf>
    ) -> CaciConfig {
        return CaciConfig {
            vcs_agent,
            script_paths,
            hooks: Vec::new()
        };
    }

    pub fn from_vcs_agent(vcs_agent: VcsAgent) -> CaciConfig {
        return CaciConfig {
            vcs_agent,
            ..CaciConfig::default()
        };
    }

    pub fn with_hooks(
        vcs_agent: VcsAgent,
        script_paths: Vec<PathBuf>,
        hooks: Vec<Hook>
    ) -> CaciConfig {
        return CaciConfig {
            vcs_agent,
            script_paths,
            hooks
        };
    }

    pub fn try_serialize(&self) -> CaciResult<String> {
        return Ok(toml_edit::ser::to_string_pretty(self)?);
    }

    pub fn try_deserialize(config_content: &str) -> CaciResult<CaciConfig> {
        return Ok(toml_edit::de::from_str(config_content)?);
    }
}

impl Default for CaciConfig {
    fn default() -> Self {
        return Self::new(
            VcsAgent::Native,
            vec![PathBuf::from(".caci/scripts")]
        );
    }
}
