use super::args::ShellType;

pub fn generate_shell_init(shell: ShellType, cmd: &str) {
    let output = match shell {
        ShellType::Zsh => generate_zsh_init(cmd),
        ShellType::Bash => generate_bash_init(cmd),
        ShellType::Fish => generate_fish_init(cmd),
    };
    print!("{}", output);
}

fn generate_zsh_init(cmd: &str) -> String {
    format!(include_str!("../shells/init.zsh"), cmd = cmd)
}

fn generate_bash_init(_cmd: &str) -> String {
    "# Bash support coming soon\n".to_string()
}

fn generate_fish_init(_cmd: &str) -> String {
    "# Fish support coming soon\n".to_string()
}
