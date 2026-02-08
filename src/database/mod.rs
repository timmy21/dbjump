pub mod clickhouse;
pub mod executor;
pub mod postgresql;
pub mod types;

pub use clickhouse::ClickHouseConnector;
pub use executor::execute_connection;
pub use postgresql::PostgreSQLConnector;
pub use types::DatabaseConnector;

use crate::config::DatabaseEngine;

pub fn get_connector(engine: &DatabaseEngine) -> Box<dyn DatabaseConnector> {
    match engine {
        DatabaseEngine::ClickHouse => Box::new(ClickHouseConnector),
        DatabaseEngine::PostgreSQL => Box::new(PostgreSQLConnector),
    }
}
