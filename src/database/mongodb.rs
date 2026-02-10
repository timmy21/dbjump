use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::Result;

pub struct MongoDBConnector;

/// Percent-encode a string for use in a MongoDB connection URI.
/// Encodes characters that are special in URIs: : / ? # [ ] @
fn uri_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b':' => encoded.push_str("%3A"),
            b'/' => encoded.push_str("%2F"),
            b'?' => encoded.push_str("%3F"),
            b'#' => encoded.push_str("%23"),
            b'[' => encoded.push_str("%5B"),
            b']' => encoded.push_str("%5D"),
            b'@' => encoded.push_str("%40"),
            b'%' => encoded.push_str("%25"),
            _ => encoded.push(byte as char),
        }
    }
    encoded
}

impl MongoDBConnector {
    fn build_connection_string(&self, config: &DatabaseConfig) -> Option<String> {
        // If no connection params at all, return None to use mongosh defaults
        if config.host.is_none()
            && config.port.is_none()
            && config.user.is_none()
            && config.password.is_none()
            && config.database.is_none()
        {
            return None;
        }

        let mut uri = String::from("mongodb://");

        // User/password credentials (percent-encoded)
        if let Some(ref user) = config.user {
            uri.push_str(&uri_encode(user));
            if let Some(ref password) = config.password {
                uri.push(':');
                uri.push_str(&uri_encode(password));
            }
            uri.push('@');
        }

        // Host and port
        let host = config.host.as_deref().unwrap_or("localhost");
        uri.push_str(host);

        if let Some(port) = config.port {
            uri.push(':');
            uri.push_str(&port.to_string());
        }

        // Database
        if let Some(ref database) = config.database {
            uri.push('/');
            uri.push_str(database);
        }

        Some(uri)
    }
}

impl DatabaseConnector for MongoDBConnector {
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command> {
        let mut cmd = Command::new(self.cli_tool_name());

        if let Some(conn_string) = self.build_connection_string(config) {
            cmd.arg(&conn_string);
        }

        // Additional options
        for option in &config.options {
            cmd.arg(option);
        }

        Ok(cmd)
    }

    fn cli_tool_name(&self) -> &str {
        "mongosh"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseEngine;

    fn create_test_config() -> DatabaseConfig {
        DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::MongoDB,
            host: Some("localhost".to_string()),
            port: Some(27017),
            user: Some("admin".to_string()),
            password: Some("secret".to_string()),
            database: Some("mydb".to_string()),
            options: vec![],
        }
    }

    #[test]
    fn test_build_connection_string_full() {
        let connector = MongoDBConnector;
        let config = create_test_config();
        let uri = connector.build_connection_string(&config).unwrap();
        assert_eq!(uri, "mongodb://admin:secret@localhost:27017/mydb");
    }

    #[test]
    fn test_build_connection_string_no_params() {
        let connector = MongoDBConnector;
        let config = DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::MongoDB,
            host: None,
            port: None,
            user: None,
            password: None,
            database: None,
            options: vec![],
        };
        assert!(connector.build_connection_string(&config).is_none());
    }

    #[test]
    fn test_build_connection_string_host_only() {
        let connector = MongoDBConnector;
        let config = DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::MongoDB,
            host: Some("myhost".to_string()),
            port: None,
            user: None,
            password: None,
            database: None,
            options: vec![],
        };
        let uri = connector.build_connection_string(&config).unwrap();
        assert_eq!(uri, "mongodb://myhost");
    }

    #[test]
    fn test_build_command() {
        let connector = MongoDBConnector;
        let config = create_test_config();

        let cmd = connector.build_command(&config).unwrap();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        assert!(args.contains(&std::ffi::OsStr::new(
            "mongodb://admin:secret@localhost:27017/mydb"
        )));
    }

    #[test]
    fn test_build_connection_string_special_chars() {
        let connector = MongoDBConnector;
        let config = DatabaseConfig {
            alias: "test".to_string(),
            engine: DatabaseEngine::MongoDB,
            host: Some("localhost".to_string()),
            port: Some(27017),
            user: Some("user@domain".to_string()),
            password: Some("p@ss:word/123".to_string()),
            database: None,
            options: vec![],
        };
        let uri = connector.build_connection_string(&config).unwrap();
        assert_eq!(uri, "mongodb://user%40domain:p%40ss%3Aword%2F123@localhost:27017");
    }
}
