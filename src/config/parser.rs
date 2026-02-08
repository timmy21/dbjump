use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::{DbJumpError, Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub alias: String,
    pub engine: DatabaseEngine,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(default)]
    pub database: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    ClickHouse,
    PostgreSQL,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|_| {
            DbJumpError::ConfigNotFound(path.as_ref().display().to_string())
        })?;

        toml::from_str(&content)
            .map_err(|e| DbJumpError::ConfigParseError(e.to_string()))
    }

    pub fn find_by_alias(&self, alias: &str) -> Result<&DatabaseConfig> {
        self.database
            .iter()
            .find(|db| db.alias == alias)
            .ok_or_else(|| DbJumpError::AliasNotFound(alias.to_string()))
    }

    pub fn get_all_aliases(&self) -> Vec<String> {
        self.database.iter().map(|db| db.alias.clone()).collect()
    }
}

impl DatabaseConfig {
    pub fn format_info(&self, hide_password: bool) -> String {
        let password_display = if hide_password {
            "***".to_string()
        } else {
            self.password.clone()
        };

        let db_display = self
            .database
            .as_ref()
            .map(|d| format!("\n  Database: {}", d))
            .unwrap_or_default();

        let options_display = if !self.options.is_empty() {
            format!("\n  Options: {}", self.options.join(" "))
        } else {
            String::new()
        };

        format!(
            "Alias: {}\n  Engine: {:?}\n  Host: {}\n  Port: {}\n  User: {}\n  Password: {}{}{}",
            self.alias,
            self.engine,
            self.host,
            self.port,
            self.user,
            password_display,
            db_display,
            options_display
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
[[database]]
alias = "test-db"
engine = "clickhouse"
host = "localhost"
port = 9000
user = "default"
password = "secret"
database = "mydb"
options = ["--multiline"]
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.database.len(), 1);
        assert_eq!(config.database[0].alias, "test-db");
        assert_eq!(config.database[0].engine, DatabaseEngine::ClickHouse);
    }
}
