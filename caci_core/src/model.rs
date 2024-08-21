use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::CaciResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VcsAgent {
    Native,
    Git
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookOutput {
    Stdout,
    Commit,
    Silent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookStage {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PrePush
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalHook {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub stage: HookStage,
    pub output: HookOutput
}

impl LocalHook {
    pub fn new(
        name: String,
        description: Option<String>,
        command: String,
        stage: HookStage,
        output: HookOutput
    ) -> LocalHook {
        return LocalHook {
            name,
            description,
            command,
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
    pub hook_url: String,
    pub hook_executor: String,
    pub stage: HookStage,
    pub output: HookOutput
}

impl RemoteHook {
    pub fn new(
        name: String,
        description: Option<String>,
        hook_url: String,
        hook_executor: String,
        stage: HookStage,
        output: HookOutput
    ) -> RemoteHook {
        return RemoteHook {
            name,
            description,
            hook_url,
            hook_executor,
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
    pub fn new(vcs_agent: VcsAgent, script_paths: Vec<PathBuf>) -> CaciConfig {
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
        return Self::new(VcsAgent::Native, vec![PathBuf::from("./caci/scripts")]);
    }
}
