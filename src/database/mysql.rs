use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::Result;

pub struct MySQLConnector;

impl DatabaseConnector for MySQLConnector {
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command> {
        let mut cmd = Command::new(self.cli_tool_name());

        // Password via env var to avoid process list exposure
        if let Some(ref password) = config.password {
            cmd.env("MYSQL_PWD", password);
        }

        if let Some(ref host) = config.host {
            cmd.arg("-h").arg(host);
        }

        if let Some(port) = config.port {
            cmd.arg("-P").arg(port.to_string());
        }

        if let Some(ref user) = config.user {
            cmd.arg("-u").arg(user);
        }

        // Additional options
        for option in &config.options {
            cmd.arg(option);
        }

        // Database name is a positional argument at the end
        if let Some(ref database) = config.database {
            cmd.arg(database);
        }

        Ok(cmd)
    }

    fn cli_tool_name(&self) -> &str {
        "mysql"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseEngine;

    fn create_test_config() -> DatabaseConfig {
        DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::MySQL,
            host: Some("localhost".to_string()),
            port: Some(3306),
            user: Some("root".to_string()),
            password: Some("secret".to_string()),
            database: Some("mydb".to_string()),
            options: vec![],
        }
    }

    #[test]
    fn test_build_command() {
        let connector = MySQLConnector;
        let config = create_test_config();

        let cmd = connector.build_command(&config).unwrap();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        assert!(args.contains(&std::ffi::OsStr::new("-h")));
        assert!(args.contains(&std::ffi::OsStr::new("localhost")));
        assert!(args.contains(&std::ffi::OsStr::new("-u")));
        assert!(args.contains(&std::ffi::OsStr::new("root")));
        // Database should be the last positional arg
        assert_eq!(args.last(), Some(&std::ffi::OsStr::new("mydb")));
        // Password should not be in args (passed via MYSQL_PWD env var)
        assert!(!args.contains(&std::ffi::OsStr::new("secret")));
    }
}
