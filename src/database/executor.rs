use std::process::Command;

use crate::config::DatabaseConfig;
use crate::database::types::DatabaseConnector;
use crate::error::{DbJumpError, Result};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub fn execute_connection(
    config: &DatabaseConfig,
    connector: Box<dyn DatabaseConnector>,
    extra_args: &[String],
) -> Result<()> {
    let mut cmd = connector.build_command(config)?;

    // Add extra arguments
    for arg in extra_args {
        cmd.arg(arg);
    }

    execute_command(cmd)
}

#[cfg(unix)]
fn execute_command(mut cmd: Command) -> Result<()> {
    // On Unix, use exec to replace the current process
    // This preserves the full interactive experience
    let error = cmd.exec();
    Err(DbJumpError::ExecutionError(error.to_string()))
}

#[cfg(not(unix))]
fn execute_command(mut cmd: Command) -> Result<()> {
    // On non-Unix systems, spawn and wait
    let status = cmd
        .status()
        .map_err(|e| DbJumpError::ExecutionError(e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap_or(-1);
        Err(DbJumpError::ExecutionError(format!(
            "Command exited with code {}",
            code
        )))
    }
}
