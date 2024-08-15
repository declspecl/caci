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
    Init {
        #[arg(short, long, value_name = "AGENT")]
        agent: CaciVcsAgent
    },
    New {
        #[arg(short, long, value_name = "PROJECT_NAME")]
        project_name: String,
        #[arg(short, long, value_name = "AGENT")]
        agent: CaciVcsAgent
    },
    Clean,
    Write,
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