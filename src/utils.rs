use std::path::Path;

/// Mask password for display
pub fn mask_password(password: &str) -> String {
    if password.is_empty() {
        String::new()
    } else {
        "***".to_string()
    }
}

/// Set file permissions to 600 (owner read/write only)
#[cfg(unix)]
pub fn set_permissions_600(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
pub fn set_permissions_600(_path: &Path) -> std::io::Result<()> {
    // On non-Unix systems, do nothing
    Ok(())
}

/// Set directory permissions to 700 (owner read/write/execute only)
#[cfg(unix)]
pub fn set_permissions_700(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o700);
    std::fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
pub fn set_permissions_700(_path: &Path) -> std::io::Result<()> {
    // On non-Unix systems, do nothing
    Ok(())
}
