use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dbjump",
    version,
    about = "Quick database connection manager",
    long_about = "dbjump helps you quickly connect to databases using short aliases instead of remembering connection parameters."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Connect to a database
    Connect {
        /// Database alias
        alias: String,

        /// Extra arguments to pass to the database CLI tool
        #[arg(trailing_var_arg = true)]
        extra_args: Vec<String>,
    },

    /// Initialize configuration file
    Init {
        /// Overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },

    /// List all configured databases
    List {
        /// Output format
        #[arg(short, long, value_name = "FORMAT", default_value = "text")]
        format: ListFormat,
    },

    /// Show connection information for a database
    Info {
        /// Database alias
        alias: String,
    },

    /// Validate configuration file
    Validate,

    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(value_name = "SHELL")]
        shell: clap_complete::Shell,
    },

    /// Generate shell integration code
    Shell {
        /// Shell type
        #[arg(value_name = "SHELL")]
        shell: ShellType,

        /// Custom command name for quick connect
        #[arg(long, default_value = "j")]
        cmd: String,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ListFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
}
