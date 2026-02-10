pub mod clickhouse;
pub mod executor;
pub mod mongodb;
pub mod mysql;
pub mod postgresql;
pub mod types;

pub use clickhouse::ClickHouseConnector;
pub use executor::execute_connection;
pub use mongodb::MongoDBConnector;
pub use mysql::MySQLConnector;
pub use postgresql::PostgreSQLConnector;
pub use types::DatabaseConnector;

use crate::config::DatabaseEngine;

pub fn get_connector(engine: &DatabaseEngine) -> Box<dyn DatabaseConnector> {
    match engine {
        DatabaseEngine::ClickHouse => Box::new(ClickHouseConnector),
        DatabaseEngine::PostgreSQL => Box::new(PostgreSQLConnector),
        DatabaseEngine::MySQL => Box::new(MySQLConnector),
        DatabaseEngine::MongoDB => Box::new(MongoDBConnector),
    }
}
