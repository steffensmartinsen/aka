use anyhow::{bail, Context, Result};
use semver::Version;
use std::process::Command;

const REPO: &str = "steffensmartinsen/aka";

pub fn run() -> Result<()> {
    // Detect platform → the release target triple
    let target = detect_target()?;

    let current = env!("CARGO_PKG_VERSION");

    let latest = latest_tag()?;
    let latest_trimmed = latest.trim_start_matches('v');

    let current_v = Version::parse(current)?;
    let latest_v = Version::parse(latest_trimmed)?;

    if latest_v <= current_v {
        println!("Already up to date (v{current}).");
        return Ok(());
    }

    println!("Updating from v{current} to {latest}...");

    let url = format!("https://github.com/{REPO}/releases/latest/download/aka-{target}.tar.gz");

    let dest = std::env::current_exe().context("Could not locate the running aka binary")?;

    let tmp_guard = TmpDir { path: tempdir()? };
    let tmp = &tmp_guard.path;
    let tarball = format!("{tmp}/aka.tar.gz");

    run_cmd("curl", &["-fsSL", &url, "-o", &tarball]).context("Download failed")?;
    run_cmd("tar", &["xzf", &tarball, "-C", &tmp]).context("Extraction failed")?;

    let new_bin = format!("{tmp}/aka");
    let staged = dest.with_extension("new");
    std::fs::copy(&new_bin, &staged)
        .with_context(|| format!("Failed to stage new binary at {}", staged.display()))?;
    std::fs::set_permissions(&staged, std::fs::metadata(&new_bin)?.permissions())?;
    std::fs::rename(&staged, &dest)
        .with_context(|| format!("Failed to install to {}", dest.display()))?;

    println!("Updated to {latest}.");
    Ok(())
}

fn detect_target() -> Result<String> {
    let os = std::env::consts::OS; // "linux" | "macos"
    let arch = std::env::consts::ARCH; // "x86_64" | "aarch64"
    let target = match (os, arch) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => bail!("Unsupported platform: {os}-{arch}. Build from source instead."),
    };
    Ok(target.to_string())
}

fn latest_tag() -> Result<String> {
    let out = Command::new("curl")
        .args([
            "-fsSL",
            "-o",
            "/dev/null",
            "-w",
            "%{url_effective}",
            &format!("https://github.com/{REPO}/releases/latest"),
        ])
        .output()
        .context("Failed to query latest release")?;
    if !out.status.success() {
        bail!("Could not reach GitHub to check the latest version");
    }
    let final_url = String::from_utf8_lossy(&out.stdout);
    // .../releases/tag/v0.2.0  →  v0.2.0
    let tag = final_url
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .context("Could not parse latest release tag")?;
    Ok(tag.to_string())
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to run {cmd}"))?;
    if !status.success() {
        bail!("{cmd} exited with an error");
    }
    Ok(())
}

fn tempdir() -> Result<String> {
    let out = Command::new("mktemp")
        .arg("-d")
        .output()
        .context("mktemp failed")?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Deletes the temp directory when dropped — runs on normal return or early `?` exit.
struct TmpDir {
    path: String,
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path); // best-effort; ignore errors
    }
}