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
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub database: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum DatabaseEngine {
    ClickHouse,
    PostgreSQL,
    MySQL,
    MongoDB,
}

impl DatabaseEngine {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "clickhouse" => Some(Self::ClickHouse),
            "postgresql" | "postgres" => Some(Self::PostgreSQL),
            "mysql" => Some(Self::MySQL),
            "mongodb" | "mongo" => Some(Self::MongoDB),
            _ => None,
        }
    }

    pub fn all_names() -> &'static [&'static str] {
        &["clickhouse", "postgresql", "postgres", "mysql", "mongodb", "mongo"]
    }
}

impl<'de> Deserialize<'de> for DatabaseEngine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DatabaseEngine::from_str(&s).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "unknown engine '{}', expected one of: {}",
                s,
                Self::all_names().join(", ")
            ))
        })
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                DbJumpError::ConfigNotFound(path.as_ref().display().to_string())
            }
            _ => DbJumpError::IoError(e),
        })?;

        toml::from_str(&content).map_err(|e| DbJumpError::ConfigParseError(e.to_string()))
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

impl std::fmt::Display for DatabaseEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseEngine::ClickHouse => write!(f, "clickhouse"),
            DatabaseEngine::PostgreSQL => write!(f, "postgresql"),
            DatabaseEngine::MySQL => write!(f, "mysql"),
            DatabaseEngine::MongoDB => write!(f, "mongodb"),
        }
    }
}

impl DatabaseConfig {
    pub fn format_info(&self, hide_password: bool) -> String {
        let mut lines = Vec::new();

        lines.push(format!("  Alias:    {}", self.alias));
        lines.push(format!("  Engine:   {}", self.engine));

        if let Some(ref host) = self.host {
            lines.push(format!("  Host:     {}", host));
        }

        if let Some(port) = self.port {
            lines.push(format!("  Port:     {}", port));
        }

        if let Some(ref user) = self.user {
            lines.push(format!("  User:     {}", user));
        }

        if self.password.is_some() {
            let password_display = if hide_password {
                "********"
            } else {
                self.password.as_deref().unwrap()
            };
            lines.push(format!("  Password: {}", password_display));
        }

        if let Some(ref database) = self.database {
            lines.push(format!("  Database: {}", database));
        }

        if !self.options.is_empty() {
            lines.push(format!("  Options:  {}", self.options.join(" ")));
        }

        lines.join("\n")
    }

    /// Returns a short summary like "clickhouse://user@host:port/db"
    pub fn format_summary(&self) -> String {
        let engine = self.engine.to_string();
        let user_part = self.user.as_deref().unwrap_or("");
        let host = self.host.as_deref().unwrap_or("localhost");
        let db = self.database.as_deref().unwrap_or("");

        let mut summary = format!("{}://", engine);
        if !user_part.is_empty() {
            summary.push_str(&format!("{}@", user_part));
        }
        summary.push_str(host);
        if let Some(port) = self.port {
            summary.push_str(&format!(":{}", port));
        }
        if !db.is_empty() {
            summary.push_str(&format!("/{}", db));
        }
        summary
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

    #[test]
    fn test_parse_engine_case_insensitive() {
        for engine_str in &["clickhouse", "ClickHouse", "CLICKHOUSE", "Clickhouse"] {
            let toml_str = format!(
                r#"
[[database]]
alias = "test"
engine = "{}"
"#,
                engine_str
            );
            let config: Config = toml::from_str(&toml_str).unwrap();
            assert_eq!(config.database[0].engine, DatabaseEngine::ClickHouse);
        }

        // "postgres" should also be accepted as an alias for PostgreSQL
        let toml_str = r#"
[[database]]
alias = "test"
engine = "postgres"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.database[0].engine, DatabaseEngine::PostgreSQL);
    }

    #[test]
    fn test_parse_engine_unknown() {
        let toml_str = r#"
[[database]]
alias = "test"
engine = "oracle"
"#;
        let result: std::result::Result<Config, _> = toml::from_str(toml_str);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unknown engine 'oracle'"));
    }

    #[test]
    fn test_parse_config_minimal() {
        let toml_str = r#"
[[database]]
alias = "minimal-db"
engine = "clickhouse"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.database.len(), 1);
        assert_eq!(config.database[0].alias, "minimal-db");
        assert!(config.database[0].host.is_none());
        assert!(config.database[0].port.is_none());
        assert!(config.database[0].user.is_none());
        assert!(config.database[0].password.is_none());
    }
}
