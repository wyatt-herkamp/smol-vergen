use std::{convert::Infallible, fmt::Debug, path::PathBuf};

use chrono::{DateTime, FixedOffset};
use derive_builder::Builder;
use either::Either;
use failed_access::FailedAccess;
use serde::Serialize;
use smol_vergen_core::{warn, Plugin, SmolVergenPluginItems, UnloadedPlugin};
#[cfg(feature = "cli")]
mod cli_access;
mod failed_access;
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
static BRANCH: &str = "BRANCH";
static COMMIT: &str = "COMMIT";
static COMMIT_SHORT: &str = "COMMIT_SHORT";
static COMMIT_AUTHOR: &str = "COMMIT_AUTHOR";
static COMMIT_MESSAGE: &str = "COMMIT_MESSAGE";
static COMMIT_TIMESTAMP: &str = "COMMIT_TIMESTAMP";
trait GitAcesss {
    type Error: Debug;
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
pub enum GitAccessOrFailed {
    Success(ActualGitAccess),
    Failed(FailedAccess),
}
impl GitAcesss for GitAccessOrFailed {
    type Error = Either<<ActualGitAccess as GitAcesss>::Error, Infallible>;

    fn load(_: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        unimplemented!("This should never be called")
    }

    fn get_branch(&self) -> Result<Option<String>, Self::Error> {
        match self {
            Self::Success(g) => g.get_branch().map_err(Either::Left),
            Self::Failed(f) => f.get_branch().map_err(Either::Right),
        }
    }

    fn get_commit(&self) -> Result<Option<String>, Self::Error> {
        match self {
            Self::Success(g) => g.get_commit().map_err(Either::Left),
            Self::Failed(f) => f.get_commit().map_err(Either::Right),
        }
    }

    fn get_commit_short(&self) -> Result<Option<String>, Self::Error> {
        match self {
            Self::Success(g) => g.get_commit_short().map_err(Either::Left),
            Self::Failed(f) => f.get_commit_short().map_err(Either::Right),
        }
    }

    fn get_commit_author(&self) -> Result<Option<GitAuthor>, Self::Error> {
        match self {
            Self::Success(g) => g.get_commit_author().map_err(Either::Left),
            Self::Failed(f) => f.get_commit_author().map_err(Either::Right),
        }
    }

    fn get_commit_message(&self) -> Result<Option<String>, Self::Error> {
        match self {
            Self::Success(g) => g.get_commit_message().map_err(Either::Left),
            Self::Failed(f) => f.get_commit_message().map_err(Either::Right),
        }
    }

    fn get_commit_timestamp(&self) -> Result<Option<DateTime<FixedOffset>>, Self::Error> {
        match self {
            Self::Success(g) => g.get_commit_timestamp().map_err(Either::Left),
            Self::Failed(f) => f.get_commit_timestamp().map_err(Either::Right),
        }
    }
}

#[derive(Clone, Copy, Builder)]
#[builder(default)]
#[builder(build_fn(private, name = "fallible_build"))]
#[non_exhaustive]
pub struct GitPlugin {
    /// Rather or not to check the parent directories for a git repository
    pub check_parents: bool,
    /// Will provide default values if an error occurs
    pub provide_defaults_on_error: bool,
}
impl Default for GitPlugin {
    fn default() -> Self {
        Self {
            check_parents: false,
            provide_defaults_on_error: true,
        }
    }
}
impl GitPluginBuilder {
    pub fn build(&self) -> GitPlugin {
        self.fallible_build()
            .expect("All types have default values. This should not fail")
    }
}
impl UnloadedPlugin for GitPlugin {
    fn load(
        &self,
        directory: std::path::PathBuf,
    ) -> Result<Box<dyn smol_vergen_core::Plugin>, anyhow::Error> {
        let load = if directory.join(".git").exists() {
            Some(ActualGitAccess::load(directory).map(GitAccessOrFailed::Success))
        } else if self.check_parents {
            let folder = find_folder_with_git(directory);
            if let Some(folder) = folder {
                Some(ActualGitAccess::load(folder).map(GitAccessOrFailed::Success))
            } else {
                None
            }
        } else {
            None
        };
        match load {
            Some(Ok(access)) => Ok(Box::new(InnerGitPlugin {
                git_access: access,
                provide_defaults_on_error: self.provide_defaults_on_error,
            }) as Box<dyn Plugin>),
            Some(Err(e)) => {
                if self.provide_defaults_on_error {
                    Ok(Box::new(InnerGitPlugin {
                        git_access: GitAccessOrFailed::Failed(FailedAccess),
                        provide_defaults_on_error: true,
                    }) as Box<dyn Plugin>)
                } else {
                    Err(e.into())
                }
            }
            None => {
                if self.provide_defaults_on_error {
                    Ok(Box::new(InnerGitPlugin {
                        git_access: GitAccessOrFailed::Failed(FailedAccess),
                        provide_defaults_on_error: true,
                    }) as Box<dyn Plugin>)
                } else {
                    Err(anyhow::anyhow!("No git repository found"))
                }
            }
        }
    }
}
#[doc(hidden)]
pub struct InnerGitPlugin {
    git_access: GitAccessOrFailed,
    provide_defaults_on_error: bool,
}
impl Plugin for InnerGitPlugin {
    fn run(
        &mut self,
        context: &mut smol_vergen_core::SmolVergenContext,
    ) -> Result<(), anyhow::Error> {
        let items = context.get_plugin_items("GIT");
        if self.provide_defaults_on_error {
            self.run_ignore_error(items);
        } else {
            self.run_with_err(items)?;
        }
        Ok(())
    }
}
impl InnerGitPlugin {
    fn run_ignore_error(&self, plugin_items: &mut SmolVergenPluginItems) {
        plugin_items.add_optional_item(
            BRANCH,
            self.git_access.get_branch().simplify(failed_access::branch),
        );
        plugin_items.add_optional_item(
            COMMIT,
            self.git_access.get_commit().simplify(failed_access::commit),
        );
        plugin_items.add_optional_item(
            COMMIT_SHORT,
            self.git_access
                .get_commit_short()
                .simplify(failed_access::commit_short),
        );
        plugin_items.add_optional_complex_item(
            COMMIT_AUTHOR,
            self.git_access
                .get_commit_author()
                .simplify(failed_access::commit_author),
        );
        plugin_items.add_optional_item(
            COMMIT_MESSAGE,
            self.git_access
                .get_commit_message()
                .simplify(failed_access::commit_message),
        );
        plugin_items.add_optional_item(
            COMMIT_TIMESTAMP,
            self.git_access
                .get_commit_timestamp()
                .simplify(failed_access::commit_timestamp),
        );
    }
    fn run_with_err(&self, plugin_items: &mut SmolVergenPluginItems) -> Result<(), anyhow::Error> {
        plugin_items.add_optional_item(BRANCH, self.git_access.get_branch()?);
        plugin_items.add_optional_item(COMMIT, self.git_access.get_commit()?);
        plugin_items.add_optional_item(COMMIT_SHORT, self.git_access.get_commit_short()?);
        plugin_items.add_optional_complex_item(COMMIT_AUTHOR, self.git_access.get_commit_author()?);
        plugin_items.add_optional_item(COMMIT_MESSAGE, self.git_access.get_commit_message()?);
        plugin_items.add_optional_item(COMMIT_TIMESTAMP, self.git_access.get_commit_timestamp()?);
        Ok(())
    }
}
trait SimplifyResult<T> {
    /// Simplifies an Result<Option<T>, E> to an Option<T> where Option<T> is the result of the function
    fn simplify<F>(self, or: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>;
}
impl<T, E> SimplifyResult<T> for Result<Option<T>, E>
where
    E: Debug,
{
    fn simplify<F>(self, or: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            Ok(Some(v)) => Some(v),
            Ok(None) => or(),
            Err(err) => {
                warn!("Error getting git info: {:?}", err);
                or()
            }
        }
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
