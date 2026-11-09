//! Writes each experiment run to its own file and never overwrites an earlier one.
//!
//! The path is `experiments/<name>/<UTC time>.txt`. A header gives the experiment name, the time,
//! the commit from `git describe --always --dirty` and the kernel release. A `-dirty` commit means
//! the working tree had uncommitted changes during the run. The file is opened with `create_new`,
//! so a run fails before it can replace an existing result.

use anyhow::{Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Directory, relative to the crate, that holds one subdirectory per experiment.
const RESULTS_DIR: &str = "experiments";

/// Prints `report` and saves it under a header. Fails rather than replace an earlier result.
pub fn record(name: &str, report: &str) -> Result<()> {
    let time = printed_by("date", &["-u", "+%Y-%m-%dT%H%M%SZ"])?;
    let commit = printed_by("git", &["describe", "--always", "--dirty"])?;
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")?;
    let header = format!(
        "experiment: {name}\ntime: {time}\ncommit: {commit}\nkernel: {}\n\n",
        kernel.trim()
    );

    let directory = PathBuf::from(RESULTS_DIR).join(name);
    let path = directory.join(format!("{time}.txt"));
    fs::create_dir_all(&directory)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .with_context(|| format!("{} already exists", path.display()))?;

    file.write_all(header.as_bytes())?;
    file.write_all(report.as_bytes())?;

    print!("{header}{report}");
    println!("\nsaved {}", path.display());

    Ok(())
}

/// Runs a program and returns what it printed, without the trailing newline.
fn printed_by(program: &str, arguments: &[&str]) -> Result<String> {
    let output = Command::new(program).args(arguments).output()?;
    let printed = String::from_utf8(output.stdout)?;

    Ok(printed.trim().to_string())
}
