use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CaciCli {
    #[command(subcommand)]
    pub command: CaciCommands
}

#[derive(Debug, Subcommand)]
pub enum CaciCommands {
    Init,
    New(CaciNewArgs),
    Run
}

#[derive(Debug, Args)]
pub struct CaciNewArgs {
    pub project_name: String
}