use std::collections::HashSet;

use crate::config::Config;
use crate::database::get_connector;
use crate::error::{DbJumpError, Result};

pub fn validate_config(config: &Config) -> Result<()> {
    let mut aliases = HashSet::new();

    for db in &config.connection {
        // Check alias uniqueness
        if !aliases.insert(&db.alias) {
            return Err(DbJumpError::DuplicateAlias(db.alias.clone()));
        }

        // Check alias format (alphanumeric, hyphens, underscores only)
        if !is_valid_alias(&db.alias) {
            return Err(DbJumpError::InvalidAliasFormat(db.alias.clone()));
        }

        // Check optional string fields are not empty or whitespace-only
        if let Some(ref host) = db.host {
            if host.trim().is_empty() {
                return Err(DbJumpError::MissingField(format!(
                    "host for alias '{}' cannot be empty",
                    db.alias
                )));
            }
        }

        if let Some(ref user) = db.user {
            if user.trim().is_empty() {
                return Err(DbJumpError::MissingField(format!(
                    "user for alias '{}' cannot be empty",
                    db.alias
                )));
            }
        }

        if let Some(ref password) = db.password {
            if password.trim().is_empty() {
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

        // Check if the CLI tool for this engine is available
        let connector = get_connector(&db.engine);
        connector.check_availability()?;
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
    use crate::config::{ConnectionConfig, DatabaseEngine};

    fn create_test_config(alias: &str) -> ConnectionConfig {
        ConnectionConfig {
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
            connection: vec![create_test_config("db1"), create_test_config("db1")],
        };

        assert!(matches!(
            validate_config(&config),
            Err(DbJumpError::DuplicateAlias(_))
        ));
    }

    #[test]
    fn test_whitespace_only_fields_rejected() {
        let mut db = create_test_config("db1");
        db.host = Some("   ".to_string());
        let config = Config {
            connection: vec![db],
        };
        assert!(matches!(
            validate_config(&config),
            Err(DbJumpError::MissingField(_))
        ));

        let mut db = create_test_config("db2");
        db.user = Some("  \t ".to_string());
        let config = Config {
            connection: vec![db],
        };
        assert!(matches!(
            validate_config(&config),
            Err(DbJumpError::MissingField(_))
        ));
    }
}
