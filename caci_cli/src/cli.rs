use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CaciCli {
    #[command(subcommand)]
    pub command: CaciCommands
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CaciTemplate {
    Git
}

#[derive(Debug, Subcommand)]
pub enum CaciCommands {
    Init {
        #[arg(short, long, value_name = "PROJECT_NAME")]
        project_name: String,
        #[arg(short, long, value_name = "TEMPLATE")]
        template: Option<CaciTemplate>
    },
    New {
        #[arg(short, long, value_name = "PROJECT_NAME")]
        project_name: String,
        #[arg(short, long, value_name = "TEMPLATE")]
        template: Option<CaciTemplate>
    },
    Run { }
}