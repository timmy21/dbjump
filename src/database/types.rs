use std::process::Command;

use crate::config::DatabaseConfig;
use crate::error::Result;

pub trait DatabaseConnector {
    /// Build the command to execute
    fn build_command(&self, config: &DatabaseConfig) -> Result<Command>;

    /// Get the name of the CLI tool
    fn cli_tool_name(&self) -> &str;

    /// Check if the CLI tool is available in PATH
    fn check_availability(&self) -> Result<()>;

    /// Format a preview string for display (e.g., in fzf)
    fn format_preview(&self, config: &DatabaseConfig) -> String;
}
