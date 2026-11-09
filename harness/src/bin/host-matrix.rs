//! One host's row in a table of where the sandbox starts.
//!
//! bubblewrap needs an unprivileged user to create a user namespace, and distributions restrict
//! this in different ways. Debian kernels add `kernel.unprivileged_userns_clone`. Ubuntu 23.10 and
//! later add `kernel.apparmor_restrict_unprivileged_userns`, which denies the namespace to programs
//! without an AppArmor profile. All kernels have `user.max_user_namespaces`, and 0 disables user
//! namespaces. When the sandbox cannot start, the harness exits before its first request to the
//! model.
//!
//! We record the distribution, the bubblewrap version and these three settings, with "absent" for a
//! setting the kernel lacks. We then start the sandbox and record the user id, process count,
//! network devices and host name that a check sees. The host name matches the host's, because
//! bubblewrap copies it into the new UTS namespace unless `--hostname` is given.
//!
//! Each run writes one file, and the files from all hosts form the table. Run it once on each host,
//! from `harness/`: `cargo run --release --bin host-matrix`

#[path = "../experiment/record.rs"]
mod record;
#[path = "../sandbox.rs"]
mod sandbox;

use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Kernel settings that can stop an unprivileged user from creating a user namespace, and the
/// host name, which tells the rows of the matrix apart.
const SETTINGS: [&str; 4] = [
    "/proc/sys/kernel/apparmor_restrict_unprivileged_userns",
    "/proc/sys/kernel/unprivileged_userns_clone",
    "/proc/sys/user/max_user_namespaces",
    "/proc/sys/kernel/hostname",
];

/// What a check sees inside the sandbox, each as a name and the script that prints it.
const VIEWS: [(&str, &str); 4] = [
    ("user id inside", "id -u"),
    ("processes inside", "ls /proc | grep -c '^[0-9]'"),
    (
        "network interfaces inside",
        "tail -n +3 /proc/net/dev | cut -d: -f1 | tr -d ' '",
    ),
    ("host name inside", "cat /proc/sys/kernel/hostname"),
];

fn main() -> Result<()> {
    let mut version = Command::new("bwrap");
    version.arg("--version");

    let bubblewrap = printed_by(version);
    let mut report = String::new();

    report.push_str(&format!("distribution | {}\n", distribution()?));
    report.push_str(&format!("bubblewrap | {bubblewrap}\n"));
    report.push_str(&setting_rows());
    report.push_str(&format!("sandbox starts | {}\n", sandbox_starts()));
    report.push_str(&view_rows()?);

    record::record("host-matrix", &report)
}

/// One row for each kernel setting.
fn setting_rows() -> String {
    let mut rows = String::new();

    for setting in SETTINGS {
        rows.push_str(&format!("{setting} | {}\n", read_or_absent(setting)));
    }

    rows
}

/// One row for each view from inside the sandbox.
fn view_rows() -> Result<String> {
    let mut rows = String::new();

    for (name, script) in VIEWS {
        let command = sandbox::command(script, Path::new("/dev/null"), "empty")?;
        rows.push_str(&format!("{name} | {}\n", printed_by(command)));
    }

    Ok(rows)
}

/// The `PRETTY_NAME` line of `/etc/os-release`.
fn distribution() -> Result<String> {
    let os_release = fs::read_to_string("/etc/os-release")?;
    let line = os_release
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .unwrap_or("PRETTY_NAME=unknown");

    Ok(line.trim_start_matches("PRETTY_NAME=").replace('"', ""))
}

/// The value in a file under `/proc/sys`, or "absent" when this kernel has no such setting.
fn read_or_absent(path: &str) -> String {
    fs::read_to_string(path).map_or(String::from("absent"), |value| value.trim().to_string())
}

/// "yes", or "no" with the reason that the startup probe gives.
fn sandbox_starts() -> String {
    match sandbox::probe() {
        Ok(()) => String::from("yes"),
        Err(failure) => format!("no: {failure:#}").replace('\n', " "),
    }
}

/// What a command prints on one line, or why it could not run.
fn printed_by(mut command: Command) -> String {
    match command.output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\n', " "),
        Err(failure) => format!("cannot run: {failure}"),
    }
}
