use tauri::{
    plugin::{Builder, BuilderError, TauriPlugin},
    test::MockRuntime,
};

/// Represents a built [`TauriPlugin`].
pub struct BuiltPlugin {
    name: &'static str,
    plugin: Result<TauriPlugin<MockRuntime>, BuilderError>,
}

impl BuiltPlugin {
    /// Build a new [`TauriPlugin`].
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            plugin: Builder::<MockRuntime>::new(name).try_build(),
        }
    }

    /// Check if the plugin failed to build due to the name being reserved.
    pub fn is_reserved(&self) -> bool {
        matches!(&self.plugin, Err(BuilderError::ReservedName(name)) if name == self.name)
    }
}
