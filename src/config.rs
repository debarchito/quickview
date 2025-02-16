// SPDX-License-Identifier: MIT
//! COSMIC config.

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
  store_path: PathBuf,
}

impl Default for Config {
  fn default() -> Self {
    let store_path = BaseDirectories::with_prefix("quickview")
      .map_err(|why| tracing::warn!(%why, "error getting XDG directory for quickview; store path will be empty"))
      .ok()
      .and_then(|xdg_dir|
        xdg_dir.create_data_directory("store")
          .map_err(|why| tracing::warn!(%why, "error creating data directory 'store' for quickview; store path will be empty"))
          .ok()
      )
      .unwrap_or_default();

    Self { store_path }
  }
}
