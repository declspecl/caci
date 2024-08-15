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
    Add {
        #[arg(short, long, value_name = "HOOK_NAME")]
        name: String,
        #[arg(short, long, value_name = "HOOK_COMMAND")]
        command: String
    },
    Remove {
        #[arg(short, long, value_name = "HOOK_NAME")]
        name: String
    }
}