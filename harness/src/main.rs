//! Harness: give it a task in plain English. It asks the model for a program, runs the
//! check the model names, and sends each failure back until the check passes.
//!
//! Usage: API_KEY=... harness write a lock free spsc queue in rust that sums 1 to 100

use anyhow::{Context, Error, Result, bail, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output};
use std::{env, fs};

const URL: &str = "https://arpanpathak.taildb9a39.ts.net/v1/chat/completions";
const MODEL: &str = "Qwen3.6-35B-A3B-NVFP4";
const MAX_ATTEMPTS: u32 = 5;
const WORK_DIR: &str = "harness_out";
const SKILL: &str = include_str!("../../skills/keep-it-simple-code/SKILL.md");

const FORMAT: &str = r#"Reply with only this JSON object:
{"file_name": "main.rs", "code": "<the whole program>", "check": "<one shell command that builds and runs only that file>"}
The program must assert its own result, so a wrong answer exits with an error."#;

/// The states of one harness run.
enum State {
    /// Ask the model for a program.
    Ask,
    /// Save and check the model's reply.
    Check(String),
    /// Send the failure back to the model.
    Fix(Error),
    /// The check passed.
    Pass,
    /// Every attempt failed.
    GiveUp,
}

/// The conversation with the model and the number of attempts so far.
struct Harness {
    /// Chat completions endpoint of the model server.
    url: String,
    /// Bearer token for the model server.
    api_key: String,
    /// Every message so far, starting with the system prompt and the task.
    conversation: Vec<Value>,
    /// The number of the current attempt, from 1 to `MAX_ATTEMPTS`.
    attempt: u32,
}

/// The program the model wrote and the command that proves it works.
#[derive(Deserialize)]
struct Answer {
    /// File name inside `WORK_DIR`.
    file_name: String,
    /// The whole program.
    code: String,
    /// Shell command, run inside `WORK_DIR`, that builds and runs the program.
    check: String,
}

fn main() -> Result<()> {
    let api_key = env::var("API_KEY").context("set API_KEY")?;
    let task = env::args().skip(1).collect::<Vec<String>>().join(" ");

    fs::create_dir_all(WORK_DIR)?;
    Harness::new(String::from(URL), api_key, task).run()?;

    println!("check passed, program is in {WORK_DIR}/");

    Ok(())
}

impl Harness {
    fn new(url: String, api_key: String, task: String) -> Self {
        let conversation = vec![
            json!({ "role": "system", "content": format!("{SKILL}\n\n{FORMAT}") }),
            json!({ "role": "user", "content": task }),
        ];

        Harness {
            url,
            api_key,
            conversation,
            attempt: 0,
        }
    }

    /// Steps through the states until the check passes or the attempts run out.
    fn run(&mut self) -> Result<()> {
        let mut state = State::Ask;

        loop {
            state = match state {
                State::Ask => self.ask(),
                State::Check(reply) => self.check(reply),
                State::Fix(failure) => self.fix(&failure),
                State::Pass => return Ok(()),
                State::GiveUp => bail!("no passing program after {MAX_ATTEMPTS} attempts"),
            };
        }
    }

    fn ask(&mut self) -> State {
        self.attempt += 1;
        println!("--- attempt {}", self.attempt);

        match self.send() {
            Ok(reply) => {
                let path = format!("{WORK_DIR}/reply-{}.txt", self.attempt);
                fs::write(&path, &reply).ok();

                println!("raw reply: {path}");

                State::Check(reply)
            }
            Err(failure) => State::Fix(failure),
        }
    }

    fn check(&mut self, reply: String) -> State {
        let outcome = Answer::parse(&reply)
            .and_then(Answer::save)
            .and_then(Answer::run)
            .and_then(report);

        self.conversation
            .push(json!({ "role": "assistant", "content": reply }));

        match outcome {
            Ok(()) => State::Pass,
            Err(failure) => State::Fix(failure),
        }
    }

    fn fix(&mut self, failure: &Error) -> State {
        eprintln!("{failure:#}");

        let feedback = format!("That failed:\n{failure:#}\nFix it and reply with the JSON again.");
        self.conversation
            .push(json!({ "role": "user", "content": feedback }));

        if self.attempt == MAX_ATTEMPTS {
            return State::GiveUp;
        }

        State::Ask
    }

    /// Sends the conversation to the model and returns the reply text.
    fn send(&self) -> Result<String> {
        let request = json!({ "model": MODEL, "messages": self.conversation, "temperature": 0.2 });
        let request_path = format!("{WORK_DIR}/request-{}.json", self.attempt);
        fs::write(request_path, serde_json::to_string_pretty(&request)?)?;

        let response: Value = ureq::post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send_json(request)?
            .into_body()
            .read_json()?;

        let finish = response["choices"][0]["finish_reason"]
            .as_str()
            .unwrap_or("unknown");
        println!("finish_reason: {finish}, usage: {}", response["usage"]);

        let content = response["choices"][0]["message"]["content"].as_str();

        Ok(content.context("the reply has no content")?.to_string())
    }
}

impl Answer {
    /// Reads the JSON answer, with or without a code fence around it.
    fn parse(reply: &str) -> Result<Self> {
        let answer_json = reply
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```");

        serde_json::from_str(answer_json).context("the reply is not the JSON asked for")
    }

    /// Writes the program into `WORK_DIR`.
    fn save(self) -> Result<Self> {
        fs::write(Path::new(WORK_DIR).join(&self.file_name), &self.code)?;

        Ok(self)
    }

    /// Runs the check inside `WORK_DIR`, with its errors merged into its output.
    fn run(self) -> Result<Output> {
        let check = format!("exec 2>&1; {}", self.check);

        Ok(Command::new("sh")
            .args(["-c", &check])
            .current_dir(WORK_DIR)
            .output()?)
    }
}

/// Fails with the check's output when it exits non-zero, prints it otherwise.
fn report(output: Output) -> Result<()> {
    let printed = String::from_utf8(output.stdout)?;

    ensure!(output.status.success(), "the check failed:\n{printed}");
    println!("{printed}");

    Ok(())
}

#[cfg(test)]
mod tests;
