#![recursion_limit = "256"]
use ahash::{HashMap, HashMapExt};
use anyhow::Context;
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use serialize_to_env::SerializeToEnv;
use smol_vergen_core::{Plugin, SmolVergenContext, SmolVergenPluginItems, UnloadedPlugin};
mod serialize_to_env;

#[derive(Default)]
pub struct SmolVergenBuilder {
    pub plugins: Vec<Box<dyn UnloadedPlugin>>,
    pub directory: Option<PathBuf>,
}

impl SmolVergenBuilder {
    pub fn add_plugin<T: UnloadedPlugin + 'static>(mut self, plugin: T) -> Self {
        let plugin: Box<dyn UnloadedPlugin> = Box::new(plugin);
        self.plugins.push(plugin);
        self
    }

    pub fn build(self) -> anyhow::Result<SmolVergen> {
        let directory = self
            .directory
            .or(std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from))
            .context("Failed to get current directory")?;
        let plugins = self
            .plugins
            .into_iter()
            .map(|v| v.load(directory.clone()))
            .collect::<anyhow::Result<Vec<Box<dyn Plugin>>>>()?;
        Ok(SmolVergen {
            plugins: plugins,
            directory: directory,
            context: SmolVergenContext::default(),
        })
    }
}
pub type SmolVergenResult = anyhow::Result<()>;
pub struct SmolVergen {
    plugins: Vec<Box<dyn Plugin>>,
    directory: PathBuf,
    pub context: SmolVergenContext,
}

impl SmolVergen {
    pub fn run_on_env(&mut self) -> SmolVergenResult {
        for plugin in &mut self.plugins {
            plugin.run(&mut self.context)?;
        }
        self.save_to_env()?;
        Ok(())
    }
    pub(crate) fn save_to_map(
        &self,
        base_name: &str,
        plugin_items: &SmolVergenPluginItems,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut map = HashMap::new();
        for (key, value) in &plugin_items.items {
            let key = format!("{}_{}", base_name, key);
            value.add_to_map(&key, &mut map);
        }
        for (key, value) in &plugin_items.complex_items {
            let prefix = format!("{}_{}", base_name, key);

            let mut ser = SerializeToEnv {
                prefix,
                result: &mut map,
            };
            erased_serde::serialize(value.as_ref(), &mut ser).unwrap();
        }

        Ok(map)
    }
    pub(crate) fn save_plugin_to_env(
        &self,
        base_name: &str,
        plugin_items: &SmolVergenPluginItems,
    ) -> anyhow::Result<()> {
        for (key, value) in &plugin_items.items {
            let key = format!("{}_{}", base_name, key);
            value.add_to_env(&key);
        }
        let mut map = HashMap::new();
        for (key, value) in &plugin_items.complex_items {
            let prefix = format!("{}_{}", base_name, key);

            let mut ser = SerializeToEnv {
                prefix,
                result: &mut map,
            };
            erased_serde::serialize(value.as_ref(), &mut ser).unwrap();
        }
        for (key, value) in map {
            std::env::set_var(key, value);
        }
        Ok(())
    }
    pub(crate) fn save_to_env(&self) -> anyhow::Result<()> {
        for (plugin_id, items) in self.context.iter() {
            let base_name = format!("SMOL_VERGEN_{}", plugin_id);
            self.save_plugin_to_env(&base_name, items)?;
        }
        Ok(())
    }
    pub(crate) fn save_to_file(&self, file: PathBuf) -> anyhow::Result<()> {
        for (plugin_id, items) in self.context.iter() {
            let base_name = format!("SMOL_VERGEN_{}", plugin_id);
            self.save_plugin_to_env(&base_name, items)?;
        }
        Ok(())
    }
}
