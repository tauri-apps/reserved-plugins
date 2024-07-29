// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! [![](https://github.com/tauri-apps/tauri/blob/dev/.github/splash.png?raw=true)](https://tauri.app)
//!
//! # Reserved Tauri Plugins
//!
//! These are plugins with reserved names that are registered by us to prevent intentional or
//! unintentional confusion. the `core` plugin is reserved due to being used as a namespace for
//! built-in plugins inside the [`tauri`] crate.
//!
//! See the [plugin documentation] for developing your own plugins.
//!
//! [`tauri`]: https://crates.io/crates/tauri
//! [plugin documentation]: https://beta.tauri.app/develop/plugins/

#[test]
fn is_reserved() {
    let plugin = util::BuiltPlugin::new("tauri");
    assert!(plugin.is_reserved());
}
