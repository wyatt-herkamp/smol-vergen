use std::path::PathBuf;

use crate::GitAcesss;
use chrono::{DateTime, FixedOffset, TimeZone};
use gix::{bstr::ByteSlice, Repository};
#[derive(thiserror::Error, Debug)]
pub enum GixError {
    #[error(transparent)]
    DiscoverError(#[from] gix::discover::Error),
    #[error(transparent)]
    TraverseError(#[from] gix::reference::find::existing::Error),
    #[error(transparent)]
    UTF8Error(#[from] gix::bstr::Utf8Error),
    #[error(transparent)]
    CommitError(#[from] gix::commit::Error),
    #[error(transparent)]
    PeelError(#[from] gix::head::peel::to_commit::Error),
    #[error(transparent)]
    ShortenError(#[from] gix::id::shorten::Error),
    #[error(transparent)]
    GixObject(#[from] gix_object::decode::Error),
    #[error(transparent)]
    GitCommitObjectError(#[from] gix::object::commit::Error),
}
pub struct GitoxideAccess {
    repository: Repository,
}
impl GitAcesss for GitoxideAccess {
    type Error = GixError;

    fn load(directory: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let repository = gix::discover(directory)?;
        Ok(Self { repository })
    }

    fn get_branch(&self) -> Result<Option<String>, Self::Error> {
        let branch = self
            .repository
            .head_name()?
            .map(|v| v.shorten().to_string());
        Ok(branch)
    }

    fn get_commit(&self) -> Result<Option<String>, Self::Error> {
        let mut head = self.repository.head()?;
        let commit = head.peel_to_commit_in_place()?;
        Ok(Some(commit.id().to_string()))
    }

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error> {
        let mut head = self.repository.head()?;
        let commit = head.peel_to_commit_in_place()?;
        Ok(Some(commit.short_id()?.to_string()))
    }

    fn get_commit_author(&self) -> Result<Option<crate::GitAuthor>, Self::Error> {
        let mut head = self.repository.head()?;
        let commit = head.peel_to_commit_in_place()?;
        let author = commit.author()?;
        Ok(Some(crate::GitAuthor {
            name: author.name.as_bstr().to_str()?.to_owned(),
            email: author.email.as_bstr().to_str()?.to_owned(),
        }))
    }

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error> {
        let mut head = self.repository.head()?;
        let commit = head.peel_to_commit_in_place()?;
        Ok(Some(
            commit
                .message()?
                .title
                .as_bstr()
                .to_str()
                .map(|s| s.to_string())?,
        ))
    }

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error> {
        let mut head = self.repository.head()?;
        let commit = head.peel_to_commit_in_place()?;
        let time = commit.time()?;
        #[cfg(test)]
        println!("Time: {:?}", time);
        let offset = match time.sign {
            gix::date::time::Sign::Plus => FixedOffset::east_opt(time.offset.abs()),
            gix::date::time::Sign::Minus => FixedOffset::west_opt(time.offset.abs()),
        };
        let Some(offset) = offset else {
            #[cfg(test)]
            println!("Could not get offset");
            return Ok(None);
        };
        let datetime = offset
            .timestamp_millis_opt(time.seconds as i64 * 1000)
            .single();
        Ok(datetime)
    }
}

#[cfg(test)]

mod tests {
    use anyhow::bail;

    use crate::{find_folder_with_git, GitAcesss};

    use super::GitoxideAccess;

    #[test]
    pub fn test_timestamp() -> anyhow::Result<()> {
        let git_folder =
            find_folder_with_git(std::env::current_dir().expect("Could not get current dir"));
        let Some(git_folder) = git_folder else {
            bail!("Could not find git folder");
        };
        let git_access = GitoxideAccess::load(git_folder)?;
        let timestamp = git_access.get_commit_timestamp()?;
        let Some(timestamp) = timestamp else {
            bail!("Could not get timestamp");
        };
        println!("Timestamp: {}", timestamp);
        Ok(())
    }
}
