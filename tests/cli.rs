use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dbjump() -> assert_cmd::Command {
    cargo_bin_cmd!("dbjump").into()
}

fn config_with(content: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(&config_path, content).unwrap();
    (dir, config_path)
}

const SAMPLE_CONFIG: &str = r#"
[[database]]
alias = "test-ch"
engine = "clickhouse"
host = "10.0.0.1"
port = 9000
user = "default"

[[database]]
alias = "test-pg"
engine = "postgresql"
host = "10.0.0.2"
user = "postgres"
password = "secret"
database = "mydb"
"#;

// --- no subcommand ---

#[test]
fn no_subcommand_shows_help() {
    dbjump()
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("connect"));
}

// --- init ---

#[test]
fn init_creates_config() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("config.toml");

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration file created"));

    assert!(config_path.exists());
}

#[test]
fn init_refuses_overwrite_without_force() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn init_overwrites_with_force() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["init", "--force"])
        .assert()
        .success();
}

// --- list ---

#[test]
fn list_text_shows_table() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("ALIAS"))
        .stdout(predicate::str::contains("ENGINE"))
        .stdout(predicate::str::contains("CONNECTION"))
        .stdout(predicate::str::contains("test-ch"))
        .stdout(predicate::str::contains("test-pg"));
}

#[test]
fn list_plain_shows_aliases_only() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["list", "--format", "plain"])
        .assert()
        .success()
        .stdout("test-ch\ntest-pg\n");
}

#[test]
fn list_json_is_valid() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    let output = dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["list", "--format", "json"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("output should be valid JSON");
    assert!(json.is_array());
    assert_eq!(json.as_array().unwrap().len(), 2);
}

#[test]
fn list_empty_config() {
    let (_dir, config_path) = config_with("database = []\n");

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No databases configured"));
}

// --- info ---

#[test]
fn info_shows_details() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["info", "test-pg"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alias:"))
        .stdout(predicate::str::contains("test-pg"))
        .stdout(predicate::str::contains("postgresql"))
        .stdout(predicate::str::contains("postgres"))
        .stdout(predicate::str::contains("********"));
}

#[test]
fn info_unknown_alias() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["info", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// --- validate ---

#[test]
fn validate_valid_config() {
    let (_dir, config_path) = config_with(SAMPLE_CONFIG);

    // This may fail if clickhouse/psql are not installed, which is expected.
    // We test the config parsing part works at minimum.
    let output = dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("validate")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Either succeeds or fails because CLI tools aren't installed
    if !output.status.success() {
        assert!(stderr.contains("not found in PATH"));
    }
}

#[test]
fn validate_duplicate_alias() {
    let config = r#"
[[database]]
alias = "dup"
engine = "clickhouse"

[[database]]
alias = "dup"
engine = "postgresql"
"#;
    let (_dir, config_path) = config_with(config);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("validate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Duplicate alias"));
}

#[test]
fn validate_whitespace_field_rejected() {
    let config = r#"
[[database]]
alias = "bad"
engine = "clickhouse"
host = "   "
"#;
    let (_dir, config_path) = config_with(config);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("validate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be empty"));
}

#[test]
fn validate_invalid_alias_format() {
    let config = r#"
[[database]]
alias = "bad alias!"
engine = "clickhouse"
"#;
    let (_dir, config_path) = config_with(config);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("validate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid alias"));
}

// --- config errors ---

#[test]
fn missing_config_file() {
    dbjump()
        .env("DBJUMP_CONFIG", "/tmp/nonexistent_dbjump_config.toml")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn invalid_engine_name() {
    let config = r#"
[[database]]
alias = "test"
engine = "oracle"
"#;
    let (_dir, config_path) = config_with(config);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown engine"));
}

#[test]
fn engine_case_insensitive() {
    let config = r#"
[[database]]
alias = "test-upper"
engine = "ClickHouse"

[[database]]
alias = "test-alias"
engine = "postgres"
"#;
    let (_dir, config_path) = config_with(config);

    dbjump()
        .env("DBJUMP_CONFIG", &config_path)
        .args(["list", "--format", "plain"])
        .assert()
        .success()
        .stdout("test-upper\ntest-alias\n");
}
