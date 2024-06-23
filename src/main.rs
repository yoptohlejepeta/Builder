mod project;
mod functions;

use clap::{Parser, Subcommand};
use project::ProjectType;
use functions::get_ignore_file;

/// Command line arguments structure
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Action to perform
    #[command(subcommand)]
    command: Command,

    /// Language
    #[arg(short, long, default_value = "Python")]
    language: String,
}

/// Commands that can be executed
#[derive(Subcommand, Debug)]
enum Command {
    /// Create files for a new project
    New {
        /// Type of project to create
        #[arg(value_parser = clap::value_parser!(ProjectType))]
        project: ProjectType,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::New { project } => {
            println!("Initializing project: {}", project);
            // create gitignore file
            let _ = get_ignore_file(args.language).await; // TODO: first letter capitilize
        }
    }    
}