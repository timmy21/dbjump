use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::Result;

pub struct ClickHouseConnector;

impl DatabaseConnector for ClickHouseConnector {
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command> {
        let mut cmd = Command::new("clickhouse");
        cmd.arg("client");

        // Optional connection parameters (only add if specified)
        if let Some(ref host) = config.host {
            cmd.arg("-h").arg(host);
        }

        if let Some(port) = config.port {
            cmd.arg("--port").arg(port.to_string());
        }

        if let Some(ref user) = config.user {
            cmd.arg("-u").arg(user);
        }

        // Password via env var to avoid process list exposure
        if let Some(ref password) = config.password {
            cmd.env("CLICKHOUSE_PASSWORD", password);
        }

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
        "clickhouse"
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
            host: Some("localhost".to_string()),
            port: Some(9000),
            user: Some("default".to_string()),
            password: Some("secret".to_string()),
            database: Some("mydb".to_string()),
            options: vec!["--multiline".to_string()],
        }
    }

    #[test]
    fn test_build_command() {
        let connector = ClickHouseConnector;
        let config = create_test_config();

        let cmd = connector.build_command(&config).unwrap();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        assert!(args.contains(&std::ffi::OsStr::new("client")));
        assert!(args.contains(&std::ffi::OsStr::new("-h")));
        assert!(args.contains(&std::ffi::OsStr::new("localhost")));
        assert!(args.contains(&std::ffi::OsStr::new("--database")));
        assert!(args.contains(&std::ffi::OsStr::new("mydb")));
        assert!(args.contains(&std::ffi::OsStr::new("--multiline")));
        // Password should not be in args (passed via env var)
        assert!(!args.contains(&std::ffi::OsStr::new("--password")));
        assert!(!args.contains(&std::ffi::OsStr::new("secret")));
    }
}
