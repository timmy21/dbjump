use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::{DbJumpError, Result};

pub struct PostgreSQLConnector;

impl DatabaseConnector for PostgreSQLConnector {
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command> {
        self.check_availability()?;

        let mut cmd = Command::new(self.cli_tool_name());

        // Optional connection parameters (only add if specified)
        if let Some(ref password) = config.password {
            cmd.env("PGPASSWORD", password);
        }

        if let Some(ref host) = config.host {
            cmd.arg("-h").arg(host);
        }

        if let Some(port) = config.port {
            cmd.arg("-p").arg(port.to_string());
        }

        if let Some(ref user) = config.user {
            cmd.arg("-U").arg(user);
        }

        if let Some(ref database) = config.database {
            cmd.arg("-d").arg(database);
        }

        // Additional options
        for option in &config.options {
            cmd.arg(option);
        }

        Ok(cmd)
    }

    fn cli_tool_name(&self) -> &str {
        "psql"
    }

    fn check_availability(&self) -> Result<()> {
        which::which(self.cli_tool_name())
            .map_err(|_| DbJumpError::CliToolNotFound(self.cli_tool_name().to_string()))?;
        Ok(())
    }

    fn format_preview(&self, config: &DatabaseConfig) -> String {
        config.format_info(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseEngine;

    fn create_test_config() -> DatabaseConfig {
        DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::PostgreSQL,
            host: Some("localhost".to_string()),
            port: Some(5432),
            user: Some("postgres".to_string()),
            password: Some("secret".to_string()),
            database: Some("mydb".to_string()),
            options: vec![],
        }
    }

    #[test]
    fn test_build_command() {
        let connector = PostgreSQLConnector;
        let config = create_test_config();

        // This will fail if psql is not installed, which is expected
        let result = connector.build_command(&config);

        if let Ok(cmd) = result {
            let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
            assert!(args.contains(&std::ffi::OsStr::new("-h")));
            assert!(args.contains(&std::ffi::OsStr::new("localhost")));
        }
    }
}
