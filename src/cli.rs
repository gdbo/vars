use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run http server
    Server {
        // Set http port
        #[arg(short, long)]
        port: Option<u16>,
    },
    /// Database manager
    Db(DbArgs),
}

#[derive(Debug, Args)]
pub struct DbArgs {
    #[command(subcommand)]
    command: Option<DbCommands>,
}

#[derive(Debug, Subcommand)]
enum DbCommands {
    /// Initial database and tables
    Init,
    /// Destroy database and tables
    Destroy,
    /// Create a new migrate file
    Migrate,
    /// Run migrate update
    Upgrade,
}
