//! False passes of model-written checks, measured against a harness-written check.
//!
//! The harness accepts a program when the model's check exits with status 0. A model-written
//! check can test less than the task requires. It can build the program without running it, or
//! run it and ignore the output, and a wrong program then passes. The sandbox limits what a
//! check can do to the host and has no bearing on what it tests.
//!
//! A typed check is one the harness builds from fixed fields, such as the file to compile and the
//! expected output. The model chooses what to test, and the harness controls how it runs. Here, the
//! typed check compiles the file with `rustc`, runs it, and compares its output with the known
//! answer.
//!
//! We send 3 tasks with a single correct output, 5 times each, at the harness's temperature of 0.2.
//! Each reply gets one attempt. Both checks run on every program, and each reply is classified as
//! one of:
//!
//! - both pass;
//! - false pass: the model's check passes and the output is wrong;
//! - false fail: the model's check fails and the output is right;
//! - both fail;
//! - no JSON, or file name refused, when the harness would run no check at all.
//!
//! Run from `harness/`: `API_KEY=... cargo run --release --bin typed-check`

#[path = "../experiment/model.rs"]
mod model;
#[path = "../experiment/record.rs"]
mod record;
#[path = "../sandbox.rs"]
mod sandbox;

use anyhow::{Context, Result};
use model::Answer;
use std::collections::BTreeMap;
use std::env;

/// Tasks in plain English, each with the only right output.
const TASKS: [(&str, &str); 3] = [
    (
        "Write a Rust program that prints the sum of the integers from 1 to 100.",
        "5050",
    ),
    (
        "Write a Rust program that prints the 30th Fibonacci number, with F(1) = F(2) = 1.",
        "832040",
    ),
    (
        "Write a Rust program that prints how many primes are below 10000.",
        "1229",
    ),
];

/// Replies per task.
const SAMPLES: usize = 5;

fn main() -> Result<()> {
    let api_key = env::var("API_KEY").context("set API_KEY")?;
    sandbox::probe()?;

    let mut lines = String::new();
    let mut counts = BTreeMap::new();

    for (task, expected) in TASKS {
        lines.push_str(&samples(&api_key, task, expected, &mut counts)?);
    }

    let report = format!(
        "replies: {}\n{}\nexpected | outcome | model's check\n{lines}",
        TASKS.len() * SAMPLES,
        listed(&counts)
    );

    record::record("typed-check", &report)
}

/// Asks for `SAMPLES` replies to one task, counts their outcomes, and returns one report
/// line per reply.
fn samples(
    api_key: &str,
    task: &str,
    expected: &str,
    counts: &mut BTreeMap<&'static str, usize>,
) -> Result<String> {
    let mut lines = String::new();

    for _sample in 0..SAMPLES {
        let reply = model::ask(api_key, &model::conversation(task))?;
        let (outcome, check) = judge(&reply, expected)?;

        *counts.entry(outcome).or_insert(0) += 1;
        lines.push_str(&format!("{expected} | {outcome} | {check}\n"));
    }

    Ok(lines)
}

/// Runs both checks on one reply and names the outcome.
fn judge(reply: &str, expected: &str) -> Result<(&'static str, String)> {
    let Ok(answer) = Answer::parse(reply) else {
        return Ok(("no JSON", String::new()));
    };

    let outcome = checked_outcome(&answer, expected)?;

    Ok((outcome, answer.check))
}

/// Saves the program, as the harness would, and names the outcome of the two checks.
fn checked_outcome(answer: &Answer, expected: &str) -> Result<&'static str> {
    let Ok(()) = answer.save() else {
        return Ok("file name refused");
    };

    let model_passes = answer.model_check_passes()?;
    let typed_passes = answer.typed_check_passes(expected)?;

    Ok(outcome(model_passes, typed_passes))
}

/// The name of each combination of the model's verdict and the harness's verdict.
fn outcome(model_passes: bool, typed_passes: bool) -> &'static str {
    match (model_passes, typed_passes) {
        (true, true) => "both pass",
        (true, false) => "false pass",
        (false, true) => "false fail",
        (false, false) => "both fail",
    }
}

/// One "outcome: count" line per outcome.
fn listed(counts: &BTreeMap<&'static str, usize>) -> String {
    let mut lines = String::new();

    for (outcome, count) in counts {
        lines.push_str(&format!("{outcome}: {count}\n"));
    }

    lines
}
