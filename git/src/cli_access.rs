use chrono::{DateTime, FixedOffset};

use crate::GitAcesss;
use std::path::PathBuf;
#[derive(thiserror::Error, Debug)]
pub enum CLIError {
    #[error("Git command not found")]
    GitCommandNotFound,
    #[error("UTF8 error")]
    UTF8Error(#[from] std::string::FromUtf8Error),
}
#[derive(Default, Clone)]
pub struct CLIGitAccess {
    git_command: PathBuf,
    directory: PathBuf,
}
impl GitAcesss for CLIGitAccess {
    type Error = CLIError;
    fn load(directory: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let which = which::which("git").map_err(|_| CLIError::GitCommandNotFound)?;
        Ok(Self {
            git_command: which,
            directory,
        })
    }
    fn get_branch(&self) -> Result<Option<String>, Self::Error> {
        let output = std::process::Command::new(&self.git_command)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .current_dir(&self.directory)
            .output()
            .map_err(|_| CLIError::GitCommandNotFound)?;
        if output.status.success() {
            let branch = String::from_utf8(output.stdout)?
                .trim_end_matches('\n')
                .to_owned();
            Ok(Some(branch))
        } else {
            Ok(None)
        }
    }

    fn get_commit(&self) -> Result<Option<String>, Self::Error> {
        let output = std::process::Command::new(&self.git_command)
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(&self.directory)
            .output()
            .map_err(|_| CLIError::GitCommandNotFound)?;
        if output.status.success() {
            let commit = String::from_utf8(output.stdout)?
                .trim_end_matches('\n')
                .to_owned();
            Ok(Some(commit))
        } else {
            Ok(None)
        }
    }

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error> {
        let output = std::process::Command::new(&self.git_command)
            .arg("rev-parse")
            .arg("--short")
            .arg("HEAD")
            .current_dir(&self.directory)
            .output()
            .map_err(|_| CLIError::GitCommandNotFound)?;
        if output.status.success() {
            let commit = String::from_utf8(output.stdout)?
                .trim_end_matches('\n')
                .to_owned();
            Ok(Some(commit))
        } else {
            Ok(None)
        }
    }

    fn get_commit_author(&self) -> Result<Option<crate::GitAuthor>, Self::Error> {
        let output = std::process::Command::new(&self.git_command)
            .arg("show")
            .arg("-s")
            .arg("--format=%an <%ae>")
            .arg("HEAD")
            .current_dir(&self.directory)
            .output()
            .map_err(|_| CLIError::GitCommandNotFound)?;
        if output.status.success() {
            let author = String::from_utf8(output.stdout).unwrap();
            let mut split = author.splitn(2, " <");
            if split.clone().count() != 2 {
                return Ok(None);
            }
            let name = split.next().unwrap().to_string();
            let email = split
                .next()
                .unwrap()
                .to_string()
                .trim_end_matches(">\n")
                .to_string();
            Ok(Some(crate::GitAuthor { name, email }))
        } else {
            Ok(None)
        }
    }

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error> {
        let output = std::process::Command::new(&self.git_command)
            .arg("show")
            .arg("-s")
            .arg("--format=%s")
            .arg("HEAD")
            .current_dir(&self.directory)
            .output()
            .map_err(|_| CLIError::GitCommandNotFound)?;
        if output.status.success() {
            let message = String::from_utf8(output.stdout)?
                .trim_end_matches('\n')
                .to_owned();
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error> {
        // TODO implement
        Ok(None)
    }
}
