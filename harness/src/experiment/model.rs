//! Model requests and answer checks shared by `typed-check` and `output-injection`.
//!
//! Requests use the harness's system prompt, reply format and temperature, so replies match what
//! the harness receives. `URL` and `MODEL` duplicate the constants in `src/main.rs` and must be
//! kept in sync.
//!
//! `Answer::save` applies the harness's plain-name rule before writing to `harness_out`. Two checks
//! then run in the sandbox. One is the model's own command. The other is the typed check, which
//! compiles the file with `rustc`, runs it and compares its output with the expected answer.

use crate::sandbox;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Output;

const URL: &str = "https://arpanpathak.taildb9a39.ts.net/v1/chat/completions";
const MODEL: &str = "Qwen3.6-35B-A3B-NVFP4";
const SKILL: &str = include_str!("../../../skills/keep-it-simple-code/SKILL.md");
const WORK_DIR: &str = "harness_out";

const FORMAT: &str = r#"Reply with only this JSON object:
{"file_name": "main.rs", "code": "<the whole program>", "check": "<one shell command that builds and runs only that file>"}
The program must assert its own result, so a wrong answer exits with an error."#;

/// The check the harness writes: build the file, run it, and print only what it prints.
const TYPED_CHECK: &str =
    "rustc -O -o /work/program /work/{file_name} 2>/dev/null && /work/program";

/// The program the model wrote and the command that proves it works.
#[derive(Deserialize)]
pub struct Answer {
    /// File name inside `WORK_DIR`.
    pub file_name: String,
    /// The whole program.
    pub code: String,
    /// Shell command, run in the sandbox next to the program, that builds and runs it.
    pub check: String,
}

/// The system prompt and the task, as the harness starts every conversation.
pub fn conversation(task: &str) -> Vec<Value> {
    vec![
        json!({ "role": "system", "content": format!("{SKILL}\n\n{FORMAT}") }),
        json!({ "role": "user", "content": task }),
    ]
}

/// Sends the conversation to the model and returns the reply text.
pub fn ask(api_key: &str, conversation: &[Value]) -> Result<String> {
    let request = json!({ "model": MODEL, "messages": conversation, "temperature": 0.2 });

    let response: Value = ureq::post(URL)
        .header("Authorization", format!("Bearer {api_key}"))
        .send_json(request)?
        .into_body()
        .read_json()?;

    let content = response["choices"][0]["message"]["content"].as_str();

    Ok(content.context("the reply has no content")?.to_string())
}

impl Answer {
    /// Reads the JSON answer, with or without a code fence around it.
    pub fn parse(reply: &str) -> Result<Self> {
        let answer_json = reply
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```");

        serde_json::from_str(answer_json).context("the reply is not the JSON asked for")
    }

    /// Writes the program into `WORK_DIR` under its plain file name.
    pub fn save(&self) -> Result<()> {
        let plain_name =
            Path::new(&self.file_name).file_name() == Some(OsStr::new(&self.file_name));
        ensure!(plain_name, "not a plain file name: {:?}", self.file_name);

        fs::create_dir_all(WORK_DIR)?;
        fs::write(Path::new(WORK_DIR).join(&self.file_name), &self.code)?;

        Ok(())
    }

    /// Runs the model's own check in the sandbox and says whether it passed.
    pub fn model_check_passes(&self) -> Result<bool> {
        let script = format!("exec 2>&1; {}", self.check);

        Ok(self.run_sandboxed(&script)?.status.success())
    }

    /// Runs the harness's check in the sandbox and says whether the program printed
    /// exactly `expected`.
    pub fn typed_check_passes(&self, expected: &str) -> Result<bool> {
        let script = TYPED_CHECK.replace("{file_name}", &self.file_name);
        let output = self.run_sandboxed(&script)?;
        let printed = String::from_utf8_lossy(&output.stdout);

        Ok(output.status.success() && printed.trim() == expected)
    }

    /// Runs a script in the sandbox next to a read-only copy of the program.
    fn run_sandboxed(&self, script: &str) -> Result<Output> {
        let program = Path::new(WORK_DIR).join(&self.file_name);

        Ok(sandbox::command(script, &program, &self.file_name)?.output()?)
    }
}
