//! Runs the model's check command in a throwaway sandbox built from Linux namespaces.
//!
//! `bwrap` (bubblewrap) starts the check in new user, mount, PID, network, IPC and UTS
//! namespaces. Inside them, we mount only `/usr`, the Rust toolchain and the program file,
//! all read-only, and an empty tmpfs as the working directory. The network namespace holds
//! only a loopback interface, and the environment holds only the variables we set. Every file
//! the check writes is lost when it exits.
//!
//! `prlimit` caps CPU time, memory, file size and open files, and `timeout` stops a check
//! after 120 seconds.

use anyhow::{Context, Result, ensure};
use std::env;
use std::path::Path;
use std::process::Command;

/// Wall-clock seconds before `timeout` sends SIGTERM to the check.
const WALL_SECONDS: &str = "120";

/// Seconds after SIGTERM before `timeout` sends SIGKILL.
const KILL_AFTER: &str = "--kill-after=5";

/// CPU seconds, address space (4 GiB), largest file (64 MiB) and open files.
const LIMITS: [&str; 4] = [
    "--cpu=60",
    "--as=4294967296",
    "--fsize=67108864",
    "--nofile=256",
];

/// Working directory of the check inside the sandbox.
const SANDBOX_DIR: &str = "/work";

/// New namespaces, a new session so that the check cannot inject keystrokes into our
/// terminal, and an empty environment.
const ISOLATION: [&str; 4] = [
    "--unshare-all",
    "--die-with-parent",
    "--new-session",
    "--clearenv",
];

/// Read-only system files that compilers and the dynamic loader need, a fresh `/proc` for
/// the new PID namespace, a minimal `/dev`, and an empty `/tmp`.
#[rustfmt::skip]
const SYSTEM_FILES: &[&str] = &[
    "--ro-bind", "/usr", "/usr",
    "--symlink", "usr/bin", "/bin",
    "--symlink", "usr/lib", "/lib",
    "--symlink", "usr/sbin", "/sbin",
    "--ro-bind-try", "/etc/alternatives", "/etc/alternatives",
    "--ro-bind-try", "/etc/ld.so.cache", "/etc/ld.so.cache",
    "--proc", "/proc",
    "--dev", "/dev",
    "--tmpfs", "/tmp",
];

/// Builds the command that runs the shell script `script` next to a read-only copy of
/// `program`, which the script finds at `/work/file_name`.
#[rustfmt::skip]
pub fn command(script: &str, program: &Path, file_name: &str) -> Result<Command> {
    let home = env::var("HOME").context("set HOME")?;
    let rustup = format!("{home}/.rustup");
    let cargo_bin = format!("{home}/.cargo/bin");
    let path = format!("{cargo_bin}:/usr/bin:/bin");
    let program_inside = format!("{SANDBOX_DIR}/{file_name}");

    let mut command = Command::new("timeout");

    command
        .args([KILL_AFTER, WALL_SECONDS, "prlimit"])
        .args(LIMITS)
        .args(["--", "bwrap"])
        .args(ISOLATION)
        .args(SYSTEM_FILES)
        .args(["--ro-bind-try", &rustup, &rustup])
        .args(["--ro-bind-try", &cargo_bin, &cargo_bin])
        .args(["--setenv", "PATH", &path])
        .args(["--setenv", "HOME", SANDBOX_DIR])
        .args(["--setenv", "RUSTUP_HOME", &rustup])
        .args(["--tmpfs", SANDBOX_DIR, "--chdir", SANDBOX_DIR])
        .arg("--ro-bind").arg(program).arg(&program_inside)
        .args(["--", "sh", "-c", script]);

    Ok(command)
}

/// Fails when this host cannot start the sandbox, so that no check ever runs unconfined.
pub fn probe() -> Result<()> {
    let output = command("true", Path::new("/dev/null"), "probe")?
        .output()
        .context("cannot start timeout, prlimit or bwrap")?;

    let reason = String::from_utf8_lossy(&output.stderr);
    ensure!(
        output.status.success(),
        "the sandbox cannot start on this host: {reason}"
    );

    Ok(())
}
