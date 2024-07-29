// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! [![](https://github.com/tauri-apps/tauri/blob/dev/.github/splash.png?raw=true)](https://tauri.app)
//!
//! # Tauri Plugins
//!
//! This is a reserved plugin name to prevent clashes with the `core:` prefix used for builtin
//! plugins from the [`tauri`] crate.
//! See the [plugin documentation] for developing your own plugins.
//!
//! [`tauri`]: https://crates.io/crates/tauri
//! [plugin documentation]: https://beta.tauri.app/develop/plugins/

#[test]
fn is_reserved() {
    let plugin = util::BuiltPlugin::new("core");
    assert!(plugin.is_reserved());
}
