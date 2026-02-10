use clap::Parser;
use dbjump::cli::{generate_completions, generate_shell_init, Cli, Commands};
use dbjump::config::{get_config_path, validate_config, Config};
use dbjump::database::{execute_connection, get_connector};
use dbjump::error::{DbJumpError, Result};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Connect { alias, extra_args }) => {
            let config = load_config()?;
            let db_config = config.find_by_alias(&alias)?;
            let connector = get_connector(&db_config.engine);
            execute_connection(db_config, connector.as_ref(), &extra_args)?;
            Ok(())
        }

        Some(Commands::Init { force }) => {
            let path = dbjump::config::path::init_config_file(force)?;
            println!("Configuration file created at: {}", path.display());
            println!("\nEdit the file to add your database connections.");
            println!("Then run 'dbjump validate' to check your configuration.");
            Ok(())
        }

        Some(Commands::List { format }) => {
            let config = load_config()?;
            match format {
                dbjump::cli::args::ListFormat::Text => {
                    for db in &config.database {
                        println!("{}", db.alias);
                    }
                }
                dbjump::cli::args::ListFormat::Json => {
                    let json = serde_json::to_string_pretty(&config.database)
                        .map_err(|e| DbJumpError::ConfigError(e.to_string()))?;
                    println!("{}", json);
                }
            }
            Ok(())
        }

        Some(Commands::Info { alias }) => {
            let config = load_config()?;
            let db_config = config.find_by_alias(&alias)?;
            println!("{}", db_config.format_info(true));
            Ok(())
        }

        Some(Commands::Validate) => {
            let config = load_config()?;
            validate_config(&config)?;
            println!("Configuration is valid!");
            Ok(())
        }

        Some(Commands::Completions { shell }) => {
            generate_completions(shell);
            Ok(())
        }

        Some(Commands::Shell { shell, cmd }) => {
            generate_shell_init(shell, &cmd);
            Ok(())
        }

        None => {
            Err(DbJumpError::ConfigError(
                "Please provide a command. Try 'dbjump --help'.".to_string(),
            ))
        }
    }
}

fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Err(DbJumpError::ConfigNotFound(format!(
            "{}. Run 'dbjump init' to create it.",
            config_path.display()
        )));
    }

    Config::from_file(config_path)
}
