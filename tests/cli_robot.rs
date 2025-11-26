use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

fn base_cmd() -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("cass"));
    cmd.env("CODING_AGENT_SEARCH_NO_UPDATE_PROMPT", "1");
    cmd
}

#[test]
fn robot_help_prints_contract() {
    let mut cmd = base_cmd();
    cmd.arg("--robot-help");
    cmd.assert()
        .success()
        .stdout(contains("cass --robot-help (contract v1)"))
        .stdout(contains("Exit codes: 0 ok"));
}

#[test]
fn robot_docs_schemas_topic() {
    let mut cmd = base_cmd();
    cmd.args(["robot-docs", "schemas"]);
    cmd.assert()
        .success()
        .stdout(contains("schemas:"))
        .stdout(contains("search:"))
        .stdout(contains("error:"))
        .stdout(contains("trace:"));
}

#[test]
fn color_never_has_no_ansi() {
    let mut cmd = base_cmd();
    cmd.args(["--color=never", "--robot-help"]);
    cmd.assert()
        .success()
        .stdout(contains("cass --robot-help"))
        .stdout(predicate::str::contains("\u{1b}").not());
}

#[test]
fn wrap_40_inserts_line_breaks() {
    let mut cmd = base_cmd();
    cmd.args(["--wrap", "40", "--robot-help"]);
    cmd.assert()
        .success()
        // With wrap at 40, long command examples should wrap across lines
        .stdout(contains("--robot #\nSearch with JSON output"));
}

#[test]
fn tui_bypasses_in_non_tty() {
    let mut cmd = base_cmd();
    // No subcommand provided; in test harness stdout is non-TTY so TUI should be blocked
    cmd.assert()
        .failure()
        .code(2)
        .stderr(contains("TUI is disabled"));
}

#[test]
fn search_error_writes_trace() {
    let tmp = TempDir::new().unwrap();
    let trace_path = tmp.path().join("trace.jsonl");

    let mut cmd = base_cmd();
    cmd.args([
        "--trace-file",
        trace_path.to_str().unwrap(),
        "--progress=plain",
        "search",
        "foo",
        "--json",
        "--data-dir",
        tmp.path().to_str().unwrap(),
    ]);

    let assert = cmd.assert().failure();
    let output = assert.get_output().clone();
    let code = output.status.code().expect("exit code present");
    // Accept both missing-index (3) and generic search error (9) depending on how the DB layer responds.
    assert!(matches!(code, 3 | 9), "unexpected exit code {code}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    if code == 3 {
        assert!(stderr.contains("missing-index"));
    } else {
        assert!(stderr.contains("\"kind\":\"search\""));
    }

    let trace = fs::read_to_string(&trace_path).expect("trace file exists");
    let last_line = trace.lines().last().expect("trace line present");
    let json: Value = serde_json::from_str(last_line).expect("valid trace json");
    let exit_code = json["exit_code"].as_i64().expect("exit_code present");
    assert_eq!(exit_code, code as i64);
    assert_eq!(json["contract_version"], "1");
}
