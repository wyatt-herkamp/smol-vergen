use std::path::PathBuf;

use chrono::{DateTime, FixedOffset, TimeZone};
use git2::Repository;

use crate::GitAcesss;
pub struct NativeGitAccess {
    pub repository: Repository,
}

impl GitAcesss for NativeGitAccess {
    type Error = git2::Error;
    fn load(directory: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let repository = Repository::open(directory)?;
        Ok(Self { repository })
    }
    fn get_branch(&self) -> Result<Option<String>, Self::Error> {
        let head = self.repository.head()?;
        let branch = head.shorthand();
        Ok(branch.map(|s| s.to_string()))
    }

    fn get_commit(&self) -> Result<Option<String>, Self::Error> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        Ok(Some(commit.id().to_string()))
    }

    fn get_commit_author(&self) -> Result<Option<crate::GitAuthor>, Self::Error> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        let author = commit.author();
        Ok(Some(crate::GitAuthor {
            name: author.name().map(|s| s.to_string()).unwrap_or_default(),
            email: author.email().map(|s| s.to_string()).unwrap_or_default(),
        }))
    }

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.summary().map(|s| s.to_string()))
    }

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        let time = commit.time();
        let offset = if time.offset_minutes() >= 0 {
            FixedOffset::west_opt(time.offset_minutes() * 60)
        } else {
            FixedOffset::east_opt(-time.offset_minutes() * 60)
        };
        let Some(offset) = offset else {
            return Ok(None);
        };
        let datetime = offset
            .timestamp_millis_opt(time.seconds() as i64 * 1000)
            .single();
        Ok(datetime)
    }

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit
            .as_object()
            .short_id()?
            .as_str()
            .map(|v| v.to_string()))
    }
}
