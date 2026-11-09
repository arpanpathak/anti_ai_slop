//! Variations of the escape attempts, judged by their effect on the host.
//!
//! The paper reports 12 hand-written escape attempts. Each one reaches one target through one path
//! with one program, so other paths and programs to the same target go untested. Here, we generate
//! the variations from short lists and run every combination.
//!
//! An attempt reads a secret file, creates a file, or connects to a TCP listener on the host. The
//! secret is a random string in a file named `secret`, placed in a new directory under `/tmp` and
//! in another inside the crate. A check can name those files by the plain path, through
//! `/proc/1/root`, through `/proc/self/root`, or by climbing out of `/work` with `..`. These 4 path
//! forms, crossed with 6 readers and 6 writers in both directories, give 96 attempts. The listener
//! runs on 127.0.0.1, and 4 clients with 2 host names add 8, for 104 in all.
//!
//! The verdict comes from the host, because a check controls its own output and exit status. A
//! read escaped if the secret appears in the check's output. A write escaped if the target exists
//! afterwards. A connection escaped if the listener accepted one.
//!
//! Each attempt also runs once as a plain `sh -c` outside the sandbox. About half succeed there,
//! and the judge must flag every one of them. Without this control, a judge that never fires would
//! report every attempt as contained. The other half fail outside as well, since `/proc/1/root`
//! needs root and `/work` exists only in the sandbox.
//!
//! The report gives the escape counts with and without the sandbox, then one line per attempt. Both
//! directories are removed at the end.
//!
//! Run from `harness/`: `cargo run --release --bin escape-mutations`

#[path = "../experiment/record.rs"]
mod record;
#[path = "../sandbox.rs"]
mod sandbox;

use anyhow::Result;
use std::env;
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};

/// Ways to name a host file from inside the sandbox.
const PATH_FORMS: [&str; 4] = [
    "{path}",
    "/proc/1/root{path}",
    "/proc/self/root{path}",
    "/work/../..{path}",
];

/// Ways to read a file and print it.
const READERS: [&str; 6] = [
    "cat {path}",
    "head -c 100 {path}",
    "dd if={path} status=none",
    "read line < {path}; echo $line",
    "cp {path} copy && cat copy",
    "python3 -c 'print(open(\"{path}\").read())'",
];

/// Ways to create a file or directory.
const WRITERS: [&str; 6] = [
    "echo escaped > {path}",
    "touch {path}",
    "mkdir {path}",
    "ln -s /usr {path}",
    "cp /usr/bin/true {path}",
    "python3 -c 'open(\"{path}\", \"w\").write(\"escaped\")'",
];

/// Ways to open a TCP connection to `{host}` on port `{port}`.
const CLIENTS: [&str; 4] = [
    "curl -s -m 2 http://{host}:{port}/",
    "nc -w 2 {host} {port} < /dev/null",
    "python3 -c 'import socket; socket.create_connection((\"{host}\", {port}), 2)'",
    "bash -c 'echo > /dev/tcp/{host}/{port}'",
];

/// Names of the host, as seen from the host.
const HOSTS: [&str; 2] = ["127.0.0.1", "localhost"];

/// One way to escape, and what it would leave on the host.
struct Attempt {
    /// The shell script of the check.
    script: String,
    /// What we look for on the host after the check exits.
    evidence: Evidence,
}

/// What an escape leaves on the host.
enum Evidence {
    /// The secret appears in the output of the check.
    Secret(String),
    /// A file or directory exists at this path.
    File(PathBuf),
    /// Our listener accepted a connection.
    Connection,
}

fn main() -> Result<()> {
    sandbox::probe()?;

    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let port = listener.local_addr()?.port();

    let host_dirs = [
        scratch_dir(env::temp_dir()),
        scratch_dir(env::current_dir()?),
    ];
    let secret = secret();

    let mut attempts = reads(&host_dirs, &secret)?;
    attempts.extend(writes(&host_dirs));
    attempts.extend(connections(port));

    let report = run_all(&attempts, &host_dirs[0], &listener)?;

    remove_scratch_dirs(&host_dirs)?;

    record::record("escape-mutations", &report)
}

/// A new directory on the host for the secret and the write targets of one run.
fn scratch_dir(parent: PathBuf) -> PathBuf {
    parent.join(format!("harness-escape-{}", process::id()))
}

/// A string that appears nowhere else, so finding it in the output proves a read.
fn secret() -> String {
    let nanoseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos());

    format!("secret-{}-{nanoseconds}", process::id())
}

/// Puts the secret in a file in each directory and returns every way to read it.
fn reads(host_dirs: &[PathBuf], secret: &str) -> Result<Vec<Attempt>> {
    let templates = fill(&READERS, "{path}", &PATH_FORMS.map(String::from));
    let mut attempts = Vec::new();

    for host_dir in host_dirs {
        let secret_file = host_dir.join("secret");
        fs::create_dir_all(host_dir)?;
        fs::write(&secret_file, secret)?;

        attempts.extend(templates.iter().map(|template| Attempt {
            script: template.replace("{path}", &secret_file.display().to_string()),
            evidence: Evidence::Secret(secret.to_string()),
        }));
    }

    Ok(attempts)
}

/// Every way to create a file in each directory, each with its own target name.
fn writes(host_dirs: &[PathBuf]) -> Vec<Attempt> {
    let templates = fill(&WRITERS, "{path}", &PATH_FORMS.map(String::from));
    let mut attempts = Vec::new();

    for host_dir in host_dirs {
        attempts.extend(
            templates
                .iter()
                .enumerate()
                .map(|(number, template)| write_attempt(host_dir, number, template)),
        );
    }

    attempts
}

/// One write, aimed at a path that no other attempt uses.
fn write_attempt(host_dir: &Path, number: usize, template: &str) -> Attempt {
    let target = host_dir.join(format!("written-{number}"));

    Attempt {
        script: template.replace("{path}", &target.display().to_string()),
        evidence: Evidence::File(target),
    }
}

/// Every way to connect to our listener.
fn connections(port: u16) -> Vec<Attempt> {
    let scripts = fill(&CLIENTS, "{host}", &HOSTS.map(String::from));

    scripts
        .iter()
        .map(|script| Attempt {
            script: script.replace("{port}", &port.to_string()),
            evidence: Evidence::Connection,
        })
        .collect()
}

/// Every template with every value put in for `placeholder`.
fn fill(templates: &[&str], placeholder: &str, values: &[String]) -> Vec<String> {
    let mut filled = Vec::new();

    for template in templates {
        filled.extend(
            values
                .iter()
                .map(|value| template.replace(placeholder, value)),
        );
    }

    filled
}

/// Runs every attempt without and then with the sandbox, and returns the report.
fn run_all(attempts: &[Attempt], work_dir: &Path, listener: &TcpListener) -> Result<String> {
    let mut lines = String::new();
    let mut unconfined_escapes = 0;
    let mut sandboxed_escapes = 0;

    for attempt in attempts {
        let (unconfined, sandboxed) = run_both(attempt, work_dir, listener)?;

        unconfined_escapes += usize::from(unconfined);
        sandboxed_escapes += usize::from(sandboxed);
        lines.push_str(&format!(
            "{} | {} | {}\n",
            verdict(unconfined),
            verdict(sandboxed),
            attempt.script
        ));
    }

    let summary = format!(
        "attempts: {}\nescaped without the sandbox: {unconfined_escapes}\n\
         escaped from the sandbox: {sandboxed_escapes}\n\n",
        attempts.len()
    );

    Ok(format!(
        "{summary}without sandbox | with sandbox | script\n{lines}"
    ))
}

/// Runs one attempt as a plain shell on the host, then in the sandbox, and says whether
/// each run left evidence.
fn run_both(attempt: &Attempt, work_dir: &Path, listener: &TcpListener) -> Result<(bool, bool)> {
    let unconfined = Command::new("sh")
        .args(["-c", &attempt.script])
        .current_dir(work_dir)
        .output()?;
    let unconfined_escaped = left_evidence(attempt, &unconfined.stdout, listener)?;

    let sandboxed = sandbox::command(&attempt.script, Path::new("/dev/null"), "empty")?.output()?;
    let sandboxed_escaped = left_evidence(attempt, &sandboxed.stdout, listener)?;

    Ok((unconfined_escaped, sandboxed_escaped))
}

/// Whether the attempt left its evidence on the host.
fn left_evidence(attempt: &Attempt, printed: &[u8], listener: &TcpListener) -> Result<bool> {
    match &attempt.evidence {
        Evidence::Secret(secret) => Ok(String::from_utf8_lossy(printed).contains(secret.as_str())),
        Evidence::File(target) => remove_if_present(target),
        Evidence::Connection => Ok(listener.accept().is_ok()),
    }
}

/// Removes the file, link or empty directory at `target` and says whether one was there,
/// so that the next run of the same attempt starts from a clean host.
fn remove_if_present(target: &Path) -> Result<bool> {
    let Ok(metadata) = target.symlink_metadata() else {
        return Ok(false);
    };

    remove(target, metadata.is_dir())?;

    Ok(true)
}

/// Removes an empty directory or a file. A link is removed as a file, so its target stays.
fn remove(target: &Path, is_dir: bool) -> Result<()> {
    if is_dir {
        return Ok(fs::remove_dir(target)?);
    }

    Ok(fs::remove_file(target)?)
}

/// "ESCAPED" or "contained".
fn verdict(escaped: bool) -> &'static str {
    if escaped {
        return "ESCAPED";
    }

    "contained"
}

/// Removes the directories that held the secret and the write targets.
fn remove_scratch_dirs(host_dirs: &[PathBuf]) -> Result<()> {
    for host_dir in host_dirs {
        fs::remove_dir_all(host_dir)?;
    }

    Ok(())
}
