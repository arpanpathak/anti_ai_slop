//! Unit tests. The tests of `run` talk to a fake model server on localhost.

use super::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn harness() -> Harness {
    harness_at("http://127.0.0.1:1/unused")
}

fn harness_at(url: &str) -> Harness {
    let task = String::from("sum 1 to 100");

    Harness::new(String::from(url), String::from("test-key"), task)
}

/// Starts a server that answers one request per body, in order, and returns its URL.
fn fake_model(bodies: Vec<String>) -> Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}/v1/chat/completions", listener.local_addr()?);

    thread::spawn(move || serve(listener, bodies));

    Ok(url)
}

fn serve(listener: TcpListener, bodies: Vec<String>) -> Result<()> {
    for (stream, body) in listener.incoming().zip(bodies) {
        respond(stream?, &body)?;
    }

    Ok(())
}

/// Reads the whole request, so the client never writes into a closed socket, then answers.
fn respond(stream: TcpStream, body: &str) -> Result<()> {
    let mut reader = BufReader::new(stream);

    let headers: Vec<String> = reader
        .by_ref()
        .lines()
        .map_while(std::result::Result::ok)
        .take_while(|line| !line.is_empty())
        .collect();

    let mut request = vec![0; content_length(&headers)?];
    reader.read_exact(&mut request)?;

    let length = body.len();
    let response =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n{body}");

    reader.into_inner().write_all(response.as_bytes())?;

    Ok(())
}

fn content_length(headers: &[String]) -> Result<usize> {
    let value = headers
        .iter()
        .find_map(|header| {
            header
                .to_lowercase()
                .strip_prefix("content-length:")
                .map(String::from)
        })
        .context("the request has no Content-Length")?;

    Ok(value.trim().parse()?)
}

/// A chat completion whose message is `content`.
fn reply(content: &str) -> String {
    json!({ "choices": [{ "message": { "content": content } }] }).to_string()
}

fn answer(file_name: &str, code: &str, check: &str) -> String {
    json!({ "file_name": file_name, "code": code, "check": check }).to_string()
}

fn failure_text(state: State) -> String {
    match state {
        State::Fix(failure) => format!("{failure:#}"),
        _ => String::from("not a failure"),
    }
}

#[test]
fn parse_reads_plain_and_fenced_json() -> Result<()> {
    let plain = answer("sum.sh", "echo 1", "sh sum.sh");
    let fenced = format!("```json\n{plain}\n```");

    assert_eq!(Answer::parse(&plain)?.file_name, "sum.sh");
    assert_eq!(Answer::parse(&fenced)?.check, "sh sum.sh");

    Ok(())
}

#[test]
fn parse_rejects_text_that_is_not_json() {
    assert!(Answer::parse("here is your program").is_err());
}

#[test]
fn check_passes_when_the_program_succeeds() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let mut harness = harness();
    let state = harness.check(answer("pass.sh", "echo sum = 5050", "sh pass.sh"));

    assert!(matches!(state, State::Pass));
    assert_eq!(harness.conversation.len(), 3);

    Ok(())
}

#[test]
fn check_returns_the_output_of_a_failing_program() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let state = harness().check(answer("fail.sh", "echo sum = 0; exit 1", "sh fail.sh"));

    assert!(failure_text(state).contains("the check failed with exit status: 1:\nsum = 0"));

    Ok(())
}

#[test]
fn check_rejects_output_that_is_not_utf8() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let state = harness().check(answer("bytes.sh", "printf '\\377'", "sh bytes.sh"));

    assert!(failure_text(state).contains("invalid utf-8"));

    Ok(())
}

#[test]
fn save_rejects_a_file_name_that_leaves_the_workspace() {
    let state = harness().check(answer("../escaped.sh", "echo 1", "true"));

    assert!(failure_text(state).contains("file_name must be a plain file name"));
    assert!(!Path::new("escaped.sh").exists());
}

#[test]
fn sandbox_discards_what_the_check_writes() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let check = "echo left > left.txt && test -s left.txt";
    let state = harness().check(answer("write.sh", "echo 1", check));

    assert!(matches!(state, State::Pass));
    assert!(!Path::new(WORK_DIR).join("left.txt").exists());

    Ok(())
}

#[test]
fn sandbox_keeps_the_program_read_only() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let state = harness().check(answer("keep.sh", "echo 1", "echo changed > keep.sh"));

    assert!(failure_text(state).contains("Read-only file system"));
    assert_eq!(
        fs::read_to_string(Path::new(WORK_DIR).join("keep.sh"))?,
        "echo 1"
    );

    Ok(())
}

#[test]
fn sandbox_hides_the_host_files_and_environment() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let check = format!("test ! -e {manifest_dir}/Cargo.toml && test -z \"$CARGO_MANIFEST_DIR\"");
    let state = harness().check(answer("hide.sh", "echo 1", &check));

    assert!(matches!(state, State::Pass));

    Ok(())
}

#[test]
fn sandbox_has_only_the_loopback_interface() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let check = "test \"$(tail -n +3 /proc/net/dev | cut -d: -f1 | tr -d ' ')\" = lo";
    let state = harness().check(answer("net.sh", "echo 1", check));

    assert!(matches!(state, State::Pass));

    Ok(())
}

#[test]
fn probe_starts_the_sandbox() -> Result<()> {
    sandbox::probe()
}

#[test]
fn run_sends_every_failure_back_until_the_check_passes() -> Result<()> {
    fs::create_dir_all(WORK_DIR)?;

    let url = fake_model(vec![
        reply("here is your program"),
        reply(&answer("wrong.sh", "echo sum = 0; exit 1", "sh wrong.sh")),
        json!({ "choices": [] }).to_string(),
        String::from("not a chat completion"),
        reply(&answer("right.sh", "echo sum = 5050", "sh right.sh")),
    ])?;

    let mut harness = harness_at(&url);
    harness.run()?;

    assert_eq!(harness.attempt, MAX_ATTEMPTS);

    Ok(())
}

#[test]
fn run_gives_up_when_the_server_is_unreachable() {
    let mut harness = harness();
    let outcome = harness.run();

    assert!(outcome.is_err());
    assert_eq!(harness.attempt, MAX_ATTEMPTS);
}

#[test]
fn fix_asks_again_until_the_last_attempt() {
    let mut harness = harness();
    let failure = anyhow::anyhow!("sum = 0");

    harness.attempt = 1;
    assert!(matches!(harness.fix(&failure), State::Ask));

    harness.attempt = MAX_ATTEMPTS;
    assert!(matches!(harness.fix(&failure), State::GiveUp));
}
