pub mod args;
pub mod completions;
pub mod shell;

pub use args::{Cli, Commands};
pub use completions::generate_completions;
pub use shell::generate_shell_init;
