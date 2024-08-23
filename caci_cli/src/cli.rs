use caci_core::model::{HookOutput, HookStage, VcsAgent};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CaciCli {
    #[command(subcommand)]
    pub command: CliCommands
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliVcsAgent {
    Native,
    Git
}

impl Into<VcsAgent> for CliVcsAgent {
    fn into(self) -> VcsAgent {
        return match self {
            CliVcsAgent::Native => VcsAgent::Native,
            CliVcsAgent::Git => VcsAgent::Git
        };
    }
}

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    /// Initialize caci in the current directory
    Init { agent: CliVcsAgent },
    /// Initialize caci and the chosen VCS agent in a new directory
    New {
        agent: CliVcsAgent,
        #[arg(short, long, value_name = "PROJECT_NAME")]
        project_name: String
    },
    /// Delete all hooks
    Clean,
    /// (Over)Write the hooks defined in caci.toml
    Write,
    /// Manage caci hooks and their configurations
    #[command(subcommand)]
    Hook(CliHookCommands)
}

#[derive(Debug, Subcommand)]
pub enum CliHookCommands {
    /// Add new local or remote hooks
    #[command(subcommand)]
    Add(CliHookAddCommands),
    /// Remove a hook by name
    Remove {
        #[arg(short, long, value_name = "NAME")]
        name: String
    },
    /// Executes a set of hooks by their hook stage
    Run { stage: CliHookStage }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliHookStage {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PrePush
}

impl Into<HookStage> for CliHookStage {
    fn into(self) -> HookStage {
        return match self {
            CliHookStage::PreCommit => HookStage::PreCommit,
            CliHookStage::PrepareCommitMsg => HookStage::PrepareCommitMsg,
            CliHookStage::CommitMsg => HookStage::CommitMsg,
            CliHookStage::PostCommit => HookStage::PostCommit,
            CliHookStage::PrePush => HookStage::PrePush
        };
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliHookOutput {
    Stdout,
    CommitMsg,
    Silent
}

impl Into<HookOutput> for CliHookOutput {
    fn into(self) -> HookOutput {
        return match self {
            CliHookOutput::Stdout => HookOutput::Stdout,
            CliHookOutput::CommitMsg => HookOutput::CommitMsg,
            CliHookOutput::Silent => HookOutput::Silent
        };
    }
}

#[derive(Debug, Subcommand)]
pub enum CliHookAddCommands {
    /// Add a new local hook (defined local script)
    Local {
        #[arg(short, long, value_name = "NAME")]
        name: String,
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        #[arg(short = 'f', long, value_name = "FILENAME")]
        script_filename: String,
        #[arg(short, long, value_name = "EXECUTOR")]
        executor: String,
        #[arg(short, long, value_name = "STAGE")]
        stage: CliHookStage,
        #[arg(short, long, value_name = "OUTPUT", default_value = "stdout")]
        output: CliHookOutput
    },
    /// Add a new remote hook (pull script from URL and execute)
    Remote {
        #[arg(short, long, value_name = "NAME")]
        name: String,
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        #[arg(short = 'u', long, value_name = "URL")]
        script_url: String,
        #[arg(short = 'f', long, value_name = "FILENAME")]
        script_filename: String,
        #[arg(short, long, value_name = "EXECUTOR")]
        executor: String,
        #[arg(short, long, value_name = "STAGE")]
        stage: CliHookStage,
        #[arg(short, long, value_name = "OUTPUT", default_value = "stdout")]
        output: CliHookOutput
    }
}
