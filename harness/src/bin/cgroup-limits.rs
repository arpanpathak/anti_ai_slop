//! Process and memory limits for a whole check, from a cgroup.
//!
//! `prlimit` limits each process in a check on its own: 60 s of CPU time, 4 GiB of address space,
//! 64 MiB per file and 256 open files. A check can stay under all of them and still start hundreds
//! of processes, or fill `/work` with many files below 64 MiB each. `/work` is a tmpfs, so every
//! byte written there is held in RAM.
//!
//! A cgroup (control group) applies one limit to all of its processes together. The kernel counts
//! the group's processes and its memory, tmpfs pages included. At the process limit, `fork` fails.
//! At the memory limit, the out-of-memory killer sends SIGKILL to a process in the group.
//! `systemd-run --user --scope` starts a command in a new cgroup without root, provided systemd
//! delegates the `pids` and `memory` controllers to the user.
//!
//! We run two checks. One forks up to 500 processes and prints how many started. The other writes
//! six 60 MiB files to `/work` and prints the MiB used. Each check runs once in the current sandbox
//! and once in the same sandbox inside a scope with `TasksMax=64`, `MemoryMax=128M` and
//! `MemorySwapMax=0`. Without the swap limit, the kernel moves tmpfs pages to swap and the write
//! succeeds.
//!
//! Each report row gives the last line the check printed, its exit status and its run time. 500
//! processes or 360 MiB means the check ran to the end. A lower count or `signal: 9` means the
//! cgroup stopped it.
//!
//! Run from `harness/`: `cargo run --release --bin cgroup-limits`

#[path = "../experiment/record.rs"]
mod record;
#[path = "../sandbox.rs"]
mod sandbox;

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

/// Starts up to 500 sleeping processes and prints how many started. Python is used because
/// `sh` exits on the first failed fork, before it can print a count.
const MANY_PROCESSES: &str = r#"python3 -c '
import os, time
started = 0
for _ in range(500):
    try:
        if os.fork() == 0:
            time.sleep(5)
            os._exit(0)
        started += 1
    except OSError:
        break
print(started)
'"#;

/// Writes six files of 60 MiB to `/work`, each under the 64 MiB file limit, and prints the
/// MiB that `/work` holds afterwards.
const LARGE_WORK_DIR: &str = "for i in 1 2 3 4 5 6; do head -c 60M /dev/zero > f$i || break; done
du -sm /work | cut -f1";

/// The attacks, each with what its last line of output counts.
const ATTACKS: [(&str, &str); 2] = [
    ("processes started, of 500", MANY_PROCESSES),
    ("MiB in /work, of 360", LARGE_WORK_DIR),
];

/// A transient systemd scope with a task cap, a memory cap and no swap.
const CGROUP: [&str; 10] = [
    "--user",
    "--scope",
    "--quiet",
    "-p",
    "TasksMax=64",
    "-p",
    "MemoryMax=128M",
    "-p",
    "MemorySwapMax=0",
    "--",
];

fn main() -> Result<()> {
    sandbox::probe()?;

    let mut report = String::from("attack | sandbox only | sandbox in a cgroup\n");

    for (counted, script) in ATTACKS {
        report.push_str(&row(counted, script)?);
    }

    record::record("cgroup-limits", &report)
}

/// Runs one attack in both setups and returns its row of the report.
fn row(counted: &str, script: &str) -> Result<String> {
    let alone = sandbox::command(script, Path::new("/dev/null"), "empty")?;
    let capped = in_cgroup(sandbox::command(script, Path::new("/dev/null"), "empty")?);

    let alone_outcome = outcome(alone)?;
    let capped_outcome = outcome(capped)?;

    Ok(format!("{counted} | {alone_outcome} | {capped_outcome}\n"))
}

/// The same command, started by `systemd-run` inside a new scope.
fn in_cgroup(command: Command) -> Command {
    let mut scoped = Command::new("systemd-run");

    scoped
        .args(CGROUP)
        .arg(command.get_program())
        .args(command.get_args());

    scoped
}

/// Runs the command and describes its last line of output, its exit status and its time.
fn outcome(mut command: Command) -> Result<String> {
    let start = Instant::now();
    let output = command.output()?;
    let milliseconds = start.elapsed().as_millis();

    let printed = String::from_utf8_lossy(&output.stdout);
    let last_line = printed.lines().last().unwrap_or("nothing printed");

    Ok(format!("{last_line}, {}, {milliseconds} ms", output.status))
}
