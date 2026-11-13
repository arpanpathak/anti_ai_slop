//! Harness: give it a task in plain English. It asks the model for a program, runs the
//! check the model names inside a sandbox, and sends each failure back until the check
//! passes.
//!
//! Usage: API_KEY=... harness write a lock free spsc queue in rust that sums 1 to 100

use anyhow::{Context, Result};
use std::{env, fs};

mod api;
mod executor;
mod models;
mod sandbox;

use crate::models::Harness;

const WORK_DIR: &str = "harness_out";

fn main() -> Result<()> {
    let api_key = env::var("API_KEY").context("set API_KEY")?;
    let task = env::args().skip(1).collect::<Vec<String>>().join(" ");

    sandbox::probe()?;
    fs::create_dir_all(WORK_DIR)?;

    Harness::new(task, api_key).run()?;

    println!("check passed, program is in {WORK_DIR}/");

    Ok(())
}
