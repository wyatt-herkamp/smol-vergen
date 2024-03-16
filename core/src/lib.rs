use std::{fmt::Debug, path::PathBuf};
mod value;
use ahash::HashMap;
use erased_serde::Serialize;
use value::Value;

pub trait Plugin {
    fn run(&mut self, context: &mut SmolVergenContext) -> anyhow::Result<()>;
}
pub trait UnloadedPlugin {
    fn load(&self, directory: PathBuf) -> anyhow::Result<Box<dyn Plugin>>;
}

#[derive(Default)]
pub struct SmolVergenPluginItems {
    pub items: HashMap<String, Value>,
    pub complex_items: HashMap<String, Box<dyn Serialize>>,
}
impl Debug for SmolVergenPluginItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmolVergenPluginItems")
            .field("items", &self.items)
            .finish()
    }
}
impl SmolVergenPluginItems {
    /// Add a new item to the plugin
    pub fn add_item(&mut self, name: impl Into<String>, item: impl Into<Value>) {
        self.items.insert(name.into(), item.into());
    }
    pub fn add_optional_item<T: Into<Value>>(&mut self, name: impl Into<String>, item: Option<T>) {
        if let Some(item) = item {
            self.add_item(name, item);
        }
    }
    /// Add a new complex item to the plugin
    pub fn add_complex_item<V: Serialize + 'static>(&mut self, name: impl Into<String>, item: V) {
        let item_boxed: Box<dyn Serialize> = Box::new(item);
        self.complex_items.insert(name.into(), item_boxed);
    }
    pub fn add_optional_complex_item<V: Serialize + 'static>(
        &mut self,
        name: impl Into<String>,
        item: Option<V>,
    ) {
        if let Some(item) = item {
            self.add_complex_item(name, item);
        }
    }
}
/// Adds a key-pair to the Rustc environment
/// This is a helper function to add a key-pair to the Rustc environment
#[doc(hidden)]
pub fn add_to_env(key: &str, value: &str) {
    println!("cargo:rustc-env={}={}", key, value);
}
#[derive(Default)]
pub struct SmolVergenContext {
    items: HashMap<&'static str, SmolVergenPluginItems>,
}
impl SmolVergenContext {
    /// Get the plugin items for a given plugin
    pub fn get_plugin_items(&mut self, plugin_name: &'static str) -> &mut SmolVergenPluginItems {
        self.items
            .entry(plugin_name)
            .or_insert_with(SmolVergenPluginItems::default)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &SmolVergenPluginItems)> {
        self.items.iter().map(|(k, v)| (*k, v))
    }
}
