// SPDX-License-Identifier: MIT
//! Provides settings for quickview.

use crate::core::i18n;
use cosmic::{app::Settings, iced::Limits};
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

/// Initializes tracing and i18n.
pub fn init() {
  let fmt_subscriber = FmtSubscriber::builder()
    .with_max_level(if cfg!(debug_assertions) {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .finish();
  subscriber::set_global_default(fmt_subscriber).expect("setting default subscriber failed");

  let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
  i18n::init(&requested_languages);
}

/// Provides the launch settings for quickview.
pub fn settings() -> Settings {
  Settings::default().size_limits(Limits::NONE.min_width(360.0).min_height(180.0))
}
