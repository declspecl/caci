use caci_core::CaciResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaciVcsAgent {
    Native,
    Git
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaciHookOutput {
    Stdout,
    Commit,
    Silent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaciHookStage {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PrePush
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CaciLocalHook {
    pub name: Option<String>,
    pub description: Option<String>,
    pub command: String,
    pub stage: CaciHookStage,
    pub output: Option<CaciHookOutput>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CaciRemoteHook {
    pub name: Option<String>,
    pub description: Option<String>,
    pub hook_url: String,
    pub hook_executor: String,
    pub stage: CaciHookStage,
    pub output: Option<CaciHookOutput>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CaciHook {
    LocalHook(CaciLocalHook),
    RemoteHook(CaciRemoteHook)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CaciConfig {
    pub vcs_agent: CaciVcsAgent,
    pub hooks: Vec<CaciHookStage>
}

impl CaciConfig {
    pub fn new(vcs_agent: CaciVcsAgent) -> CaciConfig {
        return CaciConfig {
            vcs_agent,
            hooks: Vec::with_capacity(5)
        };
    }

    pub fn with_hooks(vcs_agent: CaciVcsAgent, hooks: Vec<CaciHookStage>) -> CaciConfig {
        return CaciConfig {
            vcs_agent,
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