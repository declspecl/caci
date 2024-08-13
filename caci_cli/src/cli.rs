use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct CaciCli {
    #[command(subcommand)]
    pub command: CaciCommands
}

#[derive(Debug, Subcommand)]
pub enum CaciCommands {
    Init(CaciInitArgs),
    New(CaciNewArgs),
    Run(CaciRunArgs)
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CaciTemplate {
    Git
}

#[derive(Debug, Args)]
pub struct CaciInitArgs {
    #[arg(short, long)]
    pub project_name: String,
    #[arg(short, long)]
    pub template: Option<CaciTemplate>
}

#[derive(Debug, Args)]
pub struct CaciNewArgs {
    #[arg(short, long)]
    pub project_name: String,
    #[arg(short, long)]
    pub template: Option<CaciTemplate>
}

#[derive(Debug, Args)]
pub struct CaciRunArgs {
}