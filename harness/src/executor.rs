use anyhow::{Context, Result, ensure};
use std::{ffi::OsStr, fs, path::Path, process::Output};
use crate::models::{Answer, WORK_DIR};
use crate::sandbox;

/// Handles the side effects of the agentic loop, such as saving files and running checks.
pub struct Executor;

impl Executor {
    /// Saves the files provided in the `Answer` to the workspace directory.
    ///
    /// # Arguments
    /// * `answer` - The `Answer` containing the list of files and their content.
    ///
    /// # Returns
    /// A `Result` containing the original `Answer` if successful, or an error if writing fails.
    pub fn save(answer: Answer) -> Result<Answer> {
        for entry in &answer.files {
            let plain_name = Path::new(&entry.file_name).file_name() == Some(OsStr::new(&entry.file_name));
            ensure!(
                plain_name,
                "file_name must be a plain file name, got {:?}",
                entry.file_name
            );

            fs::write(Path::new(WORK_DIR).join(&entry.file_name), &entry.code)?;
        }

        Ok(answer)
    }

    /// Executes the verification command for the first file in the `Answer`.
    ///
    /// # Arguments
    /// * `answer` - The `Answer` containing the files and their verification commands.
    ///
    /// # Returns
    /// A `Result` containing the `Output` of the command execution.
    pub fn run(answer: Answer) -> Result<Output> {
        let first = answer.files.get(0).context("no files provided")?;
        let script = format!("exec 2>&1; {}", first.check);
        let program = Path::new(WORK_DIR).join(&first.file_name);

        Ok(sandbox::command(&script, &program, &first.file_name)?.output()?)
    }

    /// Reports the outcome of the command execution.
    ///
    /// # Arguments
    /// * `output` - The `Output` from the command execution.
    ///
    /// # Returns
    /// A `Result` that is `Ok(())` if the command succeeded, or an error if it failed.
    pub fn report(output: Output) -> Result<()> {
        let printed = String::from_utf8(output.stdout)?;
        let status = output.status;

        ensure!(
            status.success(),
            "the check failed with {status}:\n{printed}"
        );
        println!("{printed}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Answer, FileEntry};
    use std::fs;

    #[test]
    fn test_save_files() {
        let answer = Answer {
            files: vec![FileEntry {
                file_name: "test_file.rs".to_string(),
                code: "fn main() {}".to_string(),
                check: "cargo run".to_string(),
            }],
        };

        // Ensure directory exists
        fs::create_dir_all(WORK_DIR).unwrap();

        let result = Executor::save(answer).unwrap();
        let content = fs::read_to_string(Path::new(WORK_DIR).join("test_file.rs")).unwrap();

        assert_eq!(content, "fn main() {}");
        assert_eq!(result.files[0].file_name, "test_file.rs");

        // Cleanup
        let _ = fs::remove_file(Path::new(WORK_DIR).join("test_file.rs"));
    }

    #[test]
    fn test_save_invalid_filename() {
        let answer = Answer {
            files: vec![FileEntry {
                file_name: "../illegal.rs".to_string(),
                code: "fn main() {}".to_string(),
                check: "cargo run".to_string(),
            }],
        };

        let result = Executor::save(answer);
        assert!(result.is_err());
    }
}
