use std::{error::Error, path::PathBuf};

use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use smol_vergen_core::{Plugin, UnloadedPlugin};
#[cfg(feature = "cli")]
mod cli_access;
#[cfg(feature = "gix")]
mod gitoxide_access;
#[cfg(feature = "git2")]
mod native_git_access;

cfg_if::cfg_if! {
    if #[cfg(feature = "cli")]{
        pub(crate) type ActualGitAccess = cli_access::CLIGitAccess;
    } else if #[cfg(feature = "gix")] {
        pub(crate) type ActualGitAccess = gitoxide_access::GitoxideAccess;
    } else if #[cfg(feature = "git2")] {
        pub(crate) type ActualGitAccess = native_git_access::NativeGitAccess;
    }else{
        compile_error!("Either Git2, Gix, or cli feature must be enabled");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GitAuthor {
    name: String,
    email: String,
}

trait GitAcesss {
    type Error: Error;
    fn load(directory: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn get_branch(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit_author(&self) -> Result<Option<GitAuthor>, Self::Error>;

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error>;

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error>;
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
            let folder = find_folder_with_git(directory);
            if let Some(folder) = folder {
                let git_access = ActualGitAccess::load(folder)?;
                Ok(Box::new(InnerGitPlugin { git_access }))
            } else {
                return Err(anyhow::anyhow!("No git repository found"));
            }
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
        items.add_optional_item("COMMIT_SHORT", self.git_access.get_commit_short()?);
        items.add_optional_complex_item("COMMIT_AUTHOR", self.git_access.get_commit_author()?);
        items.add_optional_item("COMMIT_MESSAGE", self.git_access.get_commit_message()?);
        items.add_optional_item("COMMIT_TIMESTAMP", self.git_access.get_commit_timestamp()?);
        Ok(())
    }
}
fn find_folder_with_git(base: PathBuf) -> Option<PathBuf> {
    let mut current = base;
    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}
#[cfg(test)]
mod test {
    use anyhow::Context;

    use crate::GitAcesss;

    #[test]
    pub fn test_gix() -> anyhow::Result<()> {
        #[cfg(feature = "gix")]
        print_git_info::<crate::gitoxide_access::GitoxideAccess>()?;
        #[cfg(feature = "git2")]
        print_git_info::<crate::native_git_access::NativeGitAccess>()?;
        #[cfg(feature = "cli")]
        print_git_info::<crate::cli_access::CLIGitAccess>()?;
        Ok(())
    }

    pub fn print_git_info<G: GitAcesss + Send>() -> anyhow::Result<()>
    where
        G::Error: std::error::Error + Send + Sync + 'static,
    {
        let git_access = G::load(
            super::find_folder_with_git(
                std::env::current_dir().expect("Could not get current dir"),
            )
            .context("Could not find git repository")?,
        )?;
        println!("Testing {:?}", std::any::type_name::<G>());
        println!("Branch {:?}", git_access.get_branch()?);
        println!("Commit {:?}", git_access.get_commit()?);
        println!("Commit Short {:?}", git_access.get_commit_short()?);
        println!("Author {:?}", git_access.get_commit_author()?);
        println!("Message{:?}", git_access.get_commit_message()?);
        println!("Timestamp {:?}", git_access.get_commit_timestamp()?);
        Ok(())
    }
}
