use std::env;
use std::fs;
use std::path::PathBuf;

use crate::error::{DbJumpError, Result};
use crate::utils::{set_permissions_600, set_permissions_700};

const DEFAULT_CONFIG_NAME: &str = "config.toml";
const CONFIG_DIR_NAME: &str = "dbjump";

pub fn get_config_path() -> Result<PathBuf> {
    // Check environment variable first
    if let Ok(path) = env::var("DBJUMP_CONFIG") {
        return Ok(PathBuf::from(path));
    }

    // Use ~/.config/dbjump/config.toml
    let home_dir = dirs::home_dir()
        .ok_or_else(|| DbJumpError::ConfigError("Cannot determine home directory".to_string()))?;

    let config_dir = home_dir.join(".config").join(CONFIG_DIR_NAME);

    Ok(config_dir.join(DEFAULT_CONFIG_NAME))
}

pub fn ensure_config_dir() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    let config_dir = config_path
        .parent()
        .ok_or_else(|| DbJumpError::ConfigError("Invalid config path".to_string()))?;

    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
        set_permissions_700(config_dir)?;
    }

    Ok(config_dir.to_path_buf())
}

pub fn init_config_file(force: bool) -> Result<PathBuf> {
    let config_path = get_config_path()?;

    if config_path.exists() && !force {
        return Err(DbJumpError::ConfigError(format!(
            "Configuration file already exists at {}. Use --force to overwrite.",
            config_path.display()
        )));
    }

    ensure_config_dir()?;

    let template = r#"# dbjump configuration file
# Add your database connections below
#
# Note: All connection parameters (host, port, user, password) are optional.
# If not specified, the database CLI tool will use its default values.

# Example ClickHouse connection (with all parameters):
# [[database]]
# alias = "prod-clickhouse"
# engine = "clickhouse"
# host = "192.168.1.100"
# port = 9000
# user = "admin"
# password = "secret123"
# database = "default"  # optional
# options = ["--multiline"]  # optional

# Example ClickHouse connection (using defaults):
# [[database]]
# alias = "local-clickhouse"
# engine = "clickhouse"
# # Will use clickhouse defaults: localhost:9000, user=default

# Example PostgreSQL connection:
# [[database]]
# alias = "dev-postgres"
# engine = "postgresql"
# host = "localhost"
# port = 5432
# user = "postgres"
# password = "devpass"
# database = "myapp"  # optional
# options = []  # optional

# Example PostgreSQL connection (using defaults):
# [[database]]
# alias = "local-postgres"
# engine = "postgresql"
# # Will use psql defaults: localhost:5432, user=$USER
"#;

    fs::write(&config_path, template)?;
    set_permissions_600(&config_path)?;

    Ok(config_path)
}
