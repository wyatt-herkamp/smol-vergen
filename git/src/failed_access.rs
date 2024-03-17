use std::{convert::Infallible, path::PathBuf};

use chrono::{DateTime, FixedOffset};

use crate::GitAcesss;

#[derive(Debug, Clone, Default, Copy)]
pub struct FailedAccess;
impl GitAcesss for FailedAccess {
    type Error = Infallible;

    fn load(_: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn get_branch(&self) -> Result<Option<String>, Self::Error> {
        Ok(branch())
    }

    fn get_commit(&self) -> Result<Option<String>, Self::Error> {
        Ok(Some("Unknown".to_owned()))
    }

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error> {
        Ok(commit_short())
    }

    fn get_commit_author(&self) -> Result<Option<crate::GitAuthor>, Self::Error> {
        Ok(commit_author())
    }

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error> {
        Ok(commit_message())
    }

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error> {
        Ok(commit_timestamp())
    }
}
pub(crate) fn branch() -> Option<String> {
    Some("Unknown".to_owned())
}

pub(crate) fn commit() -> Option<String> {
    Some("Unknown".to_owned())
}
pub(crate) fn commit_short() -> Option<String> {
    Some("Unknown".to_owned())
}
pub(crate) fn commit_author() -> Option<crate::GitAuthor> {
    Some(crate::GitAuthor {
        name: "Unknown".to_owned(),
        email: "Unknown".to_owned(),
    })
}
pub(crate) fn commit_message() -> Option<String> {
    Some("Unknown".to_owned())
}
pub(crate) fn commit_timestamp() -> Option<DateTime<FixedOffset>> {
    Some(DateTime::default())
}
