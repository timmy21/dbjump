use std::collections::HashSet;

use crate::config::Config;
use crate::error::{DbJumpError, Result};

pub fn validate_config(config: &Config) -> Result<()> {
    let mut aliases = HashSet::new();

    for db in &config.database {
        // Check alias uniqueness
        if !aliases.insert(&db.alias) {
            return Err(DbJumpError::DuplicateAlias(db.alias.clone()));
        }

        // Check alias format (alphanumeric, hyphens, underscores only)
        if !is_valid_alias(&db.alias) {
            return Err(DbJumpError::InvalidAliasFormat(db.alias.clone()));
        }

        // Check optional fields if provided
        if let Some(ref host) = db.host {
            if host.is_empty() {
                return Err(DbJumpError::MissingField(format!(
                    "host for alias '{}' cannot be empty",
                    db.alias
                )));
            }
        }

        if let Some(ref user) = db.user {
            if user.is_empty() {
                return Err(DbJumpError::MissingField(format!(
                    "user for alias '{}' cannot be empty",
                    db.alias
                )));
            }
        }

        if let Some(ref password) = db.password {
            if password.is_empty() {
                return Err(DbJumpError::MissingField(format!(
                    "password for alias '{}' cannot be empty",
                    db.alias
                )));
            }
        }

        // Port validation (1-65535) if provided
        if let Some(port) = db.port {
            if port == 0 {
                return Err(DbJumpError::InvalidPort(port));
            }
        }
    }

    Ok(())
}

fn is_valid_alias(alias: &str) -> bool {
    !alias.is_empty()
        && alias
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseConfig, DatabaseEngine};

    fn create_test_config(alias: &str) -> DatabaseConfig {
        DatabaseConfig {
            alias: alias.to_string(),
            engine: DatabaseEngine::ClickHouse,
            host: Some("localhost".to_string()),
            port: Some(9000),
            user: Some("user".to_string()),
            password: Some("pass".to_string()),
            database: None,
            options: vec![],
        }
    }

    #[test]
    fn test_valid_alias() {
        assert!(is_valid_alias("prod-db"));
        assert!(is_valid_alias("dev_postgres"));
        assert!(is_valid_alias("test123"));
    }

    #[test]
    fn test_invalid_alias() {
        assert!(!is_valid_alias(""));
        assert!(!is_valid_alias("prod.db"));
        assert!(!is_valid_alias("db@prod"));
        assert!(!is_valid_alias("my db"));
    }

    #[test]
    fn test_duplicate_alias_detection() {
        let config = Config {
            database: vec![create_test_config("db1"), create_test_config("db1")],
        };

        assert!(matches!(
            validate_config(&config),
            Err(DbJumpError::DuplicateAlias(_))
        ));
    }
}
