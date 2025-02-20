// SPDX-License-Identifier: MIT
//! quickview config.

use crate::app::App;
use cosmic::{
  cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
  theme, Application,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use xdg::BaseDirectories;

/// The configuration for quickview.
#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub(crate) struct QuickviewConfig {
  /// The path where quickview stores and manages the virtual machines.
  target_path: PathBuf,
  /// The theme to use for quickview.
  theme: Theme,
}

impl Default for QuickviewConfig {
  /// Default config for quickview.
  fn default() -> Self {
    let store_path = BaseDirectories::with_prefix("quickview")
      .map_err(|why| tracing::warn!(%why, "error getting XDG directory for quickview; target path will be empty"))
      .ok()
      .and_then(|xdg_dir|
        xdg_dir.create_data_directory("target")
          .map_err(|why| tracing::warn!(%why, "error creating data directory 'store' for quickview; target path will be empty"))
          .ok()
      )
      .unwrap_or_default();

    Self {
      target_path: store_path,
      theme: Theme::default(),
    }
  }
}

impl QuickviewConfig {
  /// Use config from config directory or fallback to default in case of errors.
  pub(crate) fn config() -> QuickviewConfig {
    Config::new(App::APP_ID, QuickviewConfig::VERSION)
      .map(|ctx| match QuickviewConfig::get_entry(&ctx) {
        Ok(config) => config,
        Err((whys, config)) => {
          for why in whys {
            tracing::error!(%why, "error loading app config");
          }
          config
        }
      })
      .unwrap_or_default()
  }
}

/// The theme to use for quickview.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum Theme {
  #[default]
  System,
  Dark,
  Light,
}

impl Theme {
  /// Sets the theme for quickview and returns it.
  pub(crate) fn set_theme(&self) -> theme::Theme {
    match self {
      Self::Dark => {
        let mut t = theme::system_dark();
        t.theme_type.prefer_dark(Some(true));
        t
      }
      Self::Light => {
        let mut t = theme::system_light();
        t.theme_type.prefer_dark(Some(false));
        t
      }
      Self::System => theme::system_preference(),
    }
  }
}
