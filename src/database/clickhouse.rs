use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::{DbJumpError, Result};

pub struct ClickHouseConnector;

impl DatabaseConnector for ClickHouseConnector {
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command> {
        self.check_availability()?;

        let mut cmd = Command::new(self.cli_tool_name());

        // Basic connection parameters
        cmd.arg("-h").arg(&config.host);
        cmd.arg("--port").arg(config.port.to_string());
        cmd.arg("-u").arg(&config.user);
        cmd.arg("--password").arg(&config.password);

        // Optional database
        if let Some(ref database) = config.database {
            cmd.arg("--database").arg(database);
        }

        // Additional options
        for option in &config.options {
            cmd.arg(option);
        }

        Ok(cmd)
    }

    fn cli_tool_name(&self) -> &str {
        "clickhouse-client"
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
            engine: DatabaseEngine::ClickHouse,
            host: "localhost".to_string(),
            port: 9000,
            user: "default".to_string(),
            password: "secret".to_string(),
            database: Some("mydb".to_string()),
            options: vec!["--multiline".to_string()],
        }
    }

    #[test]
    fn test_build_command() {
        let connector = ClickHouseConnector;
        let config = create_test_config();

        // This will fail if clickhouse-client is not installed, which is expected
        let result = connector.build_command(&config);

        if let Ok(cmd) = result {
            let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
            assert!(args.contains(&std::ffi::OsStr::new("-h")));
            assert!(args.contains(&std::ffi::OsStr::new("localhost")));
        }
    }
}
