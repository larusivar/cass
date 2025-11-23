use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    fs::canonicalize(PathBuf::from(name)).expect("fixture path")
}

#[test]
fn install_easy_mode_end_to_end() {
    let tar = fixture("tests/fixtures/install/coding-agent-search-vtest-linux-x86_64.tar.gz");
    let checksum = fs::read_to_string(
        "tests/fixtures/install/coding-agent-search-vtest-linux-x86_64.tar.gz.sha256",
    )
    .unwrap()
    .trim()
    .to_string();

    let temp_home = tempfile::TempDir::new().unwrap();
    let dest = tempfile::TempDir::new().unwrap();
    let fake_bin = temp_home.path().join("bin");
    fs::create_dir_all(&fake_bin).unwrap();

    // Fake nightly rustc + cargo to skip rustup.
    fs::write(
        fake_bin.join("rustc"),
        "#!/bin/sh\necho rustc 1.80.0-nightly\n",
    )
    .unwrap();
    fs::write(
        fake_bin.join("cargo"),
        "#!/bin/sh\necho cargo 1.80.0-nightly\n",
    )
    .unwrap();
    fs::write(
        fake_bin.join("sha256sum"),
        "#!/bin/sh\n/usr/bin/sha256sum \"$@\"\n",
    )
    .unwrap();
    for f in [&"rustc", &"cargo", &"sha256sum"] {
        let p = fake_bin.join(f);
        let mut perms = fs::metadata(&p).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&p, perms).unwrap();
        Command::new("chmod").arg("+x").arg(&p).status().unwrap();
    }

    let status = Command::new("timeout")
        .arg("30s")
        .arg("bash")
        .arg("install.sh")
        .arg("--easy-mode")
        .arg("--verify")
        .arg("--dest")
        .arg(dest.path())
        .env("HOME", temp_home.path())
        .env(
            "PATH",
            format!(
                "{}:{}",
                fake_bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .env("ARTIFACT_URL", format!("file://{}", tar.display()))
        .env("CHECKSUM", checksum)
        .env("RUSTUP_INIT_SKIP", "1")
        .status()
        .expect("run installer");

    assert!(status.success(), "installer should succeed");

    let bin = dest.path().join("coding-agent-search");
    assert!(bin.exists());
    Command::new(&bin)
        .arg("--help")
        .status()
        .expect("run binary");
}
