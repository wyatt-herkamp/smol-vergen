use std::{error::Error, path::PathBuf};

use chrono::{DateTime, FixedOffset, Local};
use serde::Serialize;
use smol_vergen_core::{Plugin, UnloadedPlugin};

#[cfg(all(feature = "git2", feature = "gix"))]
compile_error!("Git2 and Gix features are mutually exclusive");
#[cfg(not(any(feature = "git2", feature = "gix")))]
compile_error!("Either Git2 or Gix feature must be enabled");
mod native_git_access;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GitAuthor {
    name: String,
    email: String,
}
#[cfg(feature = "git2")]
type ActualGitAccess = native_git_access::NativeGitAccess;
trait GitAcesss {
    type Error: Error;
    fn load(directory: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn get_branch(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit_author(&self) -> Result<Option<GitAuthor>, Self::Error>;

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error>;

    fn get_is_dirty(&self) -> Result<bool, Self::Error>;
}
#[derive(Default, Clone, Copy)]
pub struct GitPlugin {
    pub check_parents: bool,
}
impl UnloadedPlugin for GitPlugin {
    fn load(
        &self,
        directory: std::path::PathBuf,
    ) -> Result<Box<dyn smol_vergen_core::Plugin>, anyhow::Error> {
        if directory.join(".git").exists() {
            let git_access = ActualGitAccess::load(directory)?;
            Ok(Box::new(InnerGitPlugin { git_access }))
        } else if self.check_parents {
            let mut current = directory;
            loop {
                if current.join(".git").exists() {
                    let git_access = ActualGitAccess::load(current)?;
                    return Ok(Box::new(InnerGitPlugin { git_access }));
                }
                if !current.pop() {
                    break;
                }
            }
            return Err(anyhow::anyhow!("No git repository found"));
        } else {
            return Err(anyhow::anyhow!("No git repository found"));
        }
    }
}
#[doc(hidden)]
pub struct InnerGitPlugin {
    git_access: ActualGitAccess,
}
impl Plugin for InnerGitPlugin {
    fn run(
        &mut self,
        context: &mut smol_vergen_core::SmolVergenContext,
    ) -> Result<(), anyhow::Error> {
        let items = context.get_plugin_items("GIT");
        items.add_optional_item("BRANCH", self.git_access.get_branch()?);
        items.add_optional_item("COMMIT", self.git_access.get_commit()?);
        items.add_optional_complex_item("COMMIT_AUTHOR", self.git_access.get_commit_author()?);
        items.add_optional_item("COMMIT_MESSAGE", self.git_access.get_commit_message()?);
        items.add_optional_item("COMMIT_TIMESTAMP", self.git_access.get_commit_timestamp()?);
        Ok(())
    }
}
