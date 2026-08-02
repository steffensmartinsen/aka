use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

const BEGIN: &str = "# >>> aka >>>";
const END: &str = "# <<< aka <<<";

const BLOCK_BODY: &str =
    "[ -f \"$HOME/.config/aka/aliases.sh\" ] && . \"$HOME/.config/aka/aliases.sh\"";

/// Which shell to wire. Detected from $SHELL, or forced via the `--shell` flag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
}

impl Shell {
    /// Reload command shown to the user after install.
    pub fn reload_cmd(&self, rc: &Path) -> String {
        format!("source {}", rc.display())
    }
}

/// Detect the shell from a $SHELL value. Pure - no env access.
pub fn detect_shell(shell_env: &str) -> Result<Shell> {
    if shell_env.contains("zsh") {
        Ok(Shell::Zsh)
    } else if shell_env.contains("bash") {
        Ok(Shell::Bash)
    } else {
        bail!("Could not detect shell from $SHELL: {}", shell_env);
    }
}

/// Resolve the rc file for a given shell + OS + home dir.
/// `existing` decides which macOS bash file already exists (injected for testing).
pub fn resolve_rc(
    shell: Shell,
    os: &str,
    home: &Path,
    existing: impl Fn(&Path) -> bool,
) -> PathBuf {
    match shell {
        Shell::Zsh => home.join(".zshrc"),
        Shell::Bash if os == "macos" => {
            // Lookup order. Target first that exists.
            for name in [".bash_profile", ".bash_login", ".profile"] {
                let path = home.join(name);
                if existing(&path) {
                    return path;
                }
            }
            home.join(".bash_profile") // Creating if not existing.
        }
        Shell::Bash => home.join(".bashrc"),
    }
}

/// Append the managed block to `rc`. Idempotent: if a block already exists,
/// leave the file unchanged. Creates the file if missing.
pub fn install_block(rc: &Path) -> Result<InstallOutcome> {
    let content = read_or_empty(rc)?;

    if content.contains(BEGIN) {
        return Ok(InstallOutcome::AlreadyPresent);
    }

    // The block owns a leading newline so it never mutates existing content.
    // If the file lacks a trailing newline, the block's leading \n provides
    // the separator; if it has one, we get one blank line before the block.
    let block = format!("\n{BEGIN}\n{BLOCK_BODY}\n{END}\n");
    let mut new = content;
    new.push_str(&block);

    std::fs::write(rc, new).with_context(|| format!("Failed to write {}", rc.display()))?;
    Ok(InstallOutcome::Installed)
}

/// Remove the managed block from `rc`, atomically. Returns whether anything was removed.
pub fn remove_block(rc: &Path) -> Result<bool> {
    if !rc.exists() {
        return Ok(false);
    }
    let content = read_or_empty(rc)?;

    let begin = match content.find(BEGIN) {
        Some(i) => i,
        None => return Ok(false), // nothing to remove
    };
    let end = content
        .find(END)
        .context("Found aka's start marker but no end marker; refusing to edit. Fix the file by hand.")?
        + END.len();

    // The block was inserted as "\n<markers>\n". Cut from the newline
    // immediately before BEGIN through END, plus a trailing newline if present.
    let cut_start = content[..begin].rfind('\n').map_or(begin, |i| i);
    let mut cut_end = end;
    if content[cut_end..].starts_with('\n') {
        cut_end += 1;
    }
    let result = format!("{}{}", &content[..cut_start], &content[cut_end..]);

    write_atomic(rc, &result)?;
    Ok(true)
}

fn read_or_empty(path: &Path) -> Result<String> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).with_context(|| format!("Failed to read {}", path.display())),
    }
}

/// Write to a temp file in the same directory, then rename over the target.
/// Same-dir guarantees same filesystem, so rename is atomic.
fn write_atomic(path: &Path, content: &str) -> Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let tmp = dir.join(".aka.tmp");
    std::fs::write(&tmp, content).with_context(|| format!("Failed to write {}", tmp.display()))?;
    std::fs::rename(&tmp, path).with_context(|| format!("Failed to replace {}", path.display()))?;
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum InstallOutcome {
    Installed,
    AlreadyPresent,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Install then remove should yield byte-indentical file.
    fn round_trip(original: &str) {
        let dir = std::env::temp_dir().join(format!("aka-test-{}", rand_suffix()));
        std::fs::create_dir_all(&dir).unwrap();
        let rc = dir.join("rc");
        std::fs::write(&rc, original).unwrap();

        install_block(&rc).unwrap();
        remove_block(&rc).unwrap();

        let after = std::fs::read_to_string(&rc).unwrap();
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(original, after, "round trip changed the file");
    }

    fn rand_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    #[test]
    fn round_trip_with_trailing_newline() {
        round_trip("export FOO=1\nalias x='ls'\n");
    }

    #[test]
    fn round_trip_without_trailing_newline() {
        round_trip("export FOO=1\nalias x='ls'"); // the bug we just hit
    }

    #[test]
    fn round_trip_empty_file() {
        round_trip("");
    }

    #[test]
    fn round_trip_crlf() {
        round_trip("export FOO=1\r\nalias x='ls'\r\n");
    }

    #[test]
    fn install_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("aka-idem-{}", rand_suffix()));
        std::fs::create_dir_all(&dir).unwrap();
        let rc = dir.join("rc");
        std::fs::write(&rc, "export FOO=1\n").unwrap();

        assert_eq!(install_block(&rc).unwrap(), InstallOutcome::Installed);
        assert_eq!(install_block(&rc).unwrap(), InstallOutcome::AlreadyPresent);

        // Only one block, even after two installs.
        let content = std::fs::read_to_string(&rc).unwrap();
        assert_eq!(content.matches(BEGIN).count(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn remove_on_file_without_block_is_noop() {
        let dir = std::env::temp_dir().join(format!("aka-noop-{}", rand_suffix()));
        std::fs::create_dir_all(&dir).unwrap();
        let rc = dir.join("rc");
        std::fs::write(&rc, "export FOO=1\n").unwrap();

        assert_eq!(remove_block(&rc).unwrap(), false);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unmatched_begin_marker_errors() {
        let dir = std::env::temp_dir().join(format!("aka-bad-{}", rand_suffix()));
        std::fs::create_dir_all(&dir).unwrap();
        let rc = dir.join("rc");
        // BEGIN present, END missing → must error, not silently repair.
        std::fs::write(&rc, "foo\n# >>> aka >>>\nsomething\n").unwrap();

        assert!(remove_block(&rc).is_err());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolve_rc_targets() {
        let home = PathBuf::from("/home/u");
        let none = |_: &Path| false;

        assert_eq!(resolve_rc(Shell::Zsh, "macos", &home, none), home.join(".zshrc"));
        assert_eq!(resolve_rc(Shell::Bash, "linux", &home, none), home.join(".bashrc"));
        // macOS bash, nothing exists → creates .bash_profile
        assert_eq!(resolve_rc(Shell::Bash, "macos", &home, none), home.join(".bash_profile"));
        // macOS bash, .profile exists → targets it (shadowing rule)
        let only_profile = |p: &Path| p.ends_with(".profile");
        assert_eq!(resolve_rc(Shell::Bash, "macos", &home, only_profile), home.join(".profile"));
    }

}