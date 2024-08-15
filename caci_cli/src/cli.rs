use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CaciCli {
    #[command(subcommand)]
    pub command: CaciCommands
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CaciVcsAgent {
    Native,
    Git
}

#[derive(Debug, Subcommand)]
pub enum CaciCommands {
    /// Initialize CACI in the current directory
    Init {
        agent: CaciVcsAgent
    },
    /// Initialize CACI and the chosen VCS agent in a new directory
    New {
        agent: CaciVcsAgent,
        #[arg(short, long, value_name = "PROJECT_NAME")]
        project_name: String
    },
    /// Delete all hooks
    Clean,
    /// (Over)Write the hooks defined in caci.toml
    Write,
    /// Manage CACI hooks and their configurations
    #[command(subcommand)]
    Hook(CaciHookCommands)
}

#[derive(Debug, Subcommand)]
pub enum CaciHookCommands {
    /// Add new local or remote hooks
    #[command(subcommand)]
    Add(CaciHookAddCommands),
    /// Remove a hook by name
    Remove {
        #[arg(short, long, value_name = "NAME")]
        name: String
    },
    /// Executes an arbitrary hook by name
    Run {
        #[arg(short, long, value_name = "NAME")]
        name: String
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CaciHookStage {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PrePush
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CaciHookOutput {
    Stdout,
    Commit,
    Silent
}

#[derive(Debug, Subcommand)]
pub enum CaciHookAddCommands {
    /// Add a new local hook (defined local script)
    Local {
        #[arg(short, long, value_name = "NAME")]
        name: String,
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        #[arg(short, long, value_name = "COMMAND")]
        command: String,
        #[arg(short, long, value_name = "STAGE")]
        stage: CaciHookStage,
        #[arg(short, long, value_name = "OUTPUT", default_value = "stdout")]
        output: CaciHookOutput
    },
    /// Add a new remote hook (pull script from URL and execute)
    Remote {
        #[arg(short, long, value_name = "NAME")]
        name: String,
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        #[arg(short, long, value_name = "HOOK_URL")]
        hook_url: String,
        #[arg(short, long, value_name = "HOOK_EXECUTOR")]
        hook_executor: String,
        #[arg(short, long, value_name = "STAGE")]
        stage: CaciHookStage,
        #[arg(short, long, value_name = "OUTPUT", default_value = "stdout")]
        output: CaciHookOutput
    }
}