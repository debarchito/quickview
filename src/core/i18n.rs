// SPDX-License-Identifier: MIT
//! Provides localization support.

use i18n_embed::{
  fluent::{fluent_language_loader, FluentLanguageLoader},
  unic_langid::LanguageIdentifier,
  DefaultLocalizer, LanguageLoader, Localizer,
};
use rust_embed::RustEmbed;
use std::sync::LazyLock;

/// Holds the localizations in the `i18n` directory.
#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

/// The global language loader.
pub(crate) static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
  let loader: FluentLanguageLoader = fluent_language_loader!();

  if let Err(why) = loader.load_fallback_language(&Localizations) {
    tracing::error!(%why, "error while loading fallback language");
  };

  loader
});

/// Get the `Localizer` to be used for localizing this library.
pub(crate) fn localizer() -> Box<dyn Localizer> {
  Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

/// Applies the requested language(s) to requested translations from the `fl!()` macro.
pub(crate) fn init(requested_languages: &[LanguageIdentifier]) {
  if let Err(why) = localizer().select(requested_languages) {
    tracing::error!(%why, "error while loading fluent localizations");
  }
}

/// Request a localized string by ID from the i18n/ directory.
#[macro_export]
macro_rules! fl {
  ($message_id:literal) => {{
    i18n_embed_fl::fl!($crate::core::i18n::LANGUAGE_LOADER, $message_id)
  }};

  ($message_id:literal, $($args:expr),*) => {{
    i18n_embed_fl::fl!($crate::core::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
  }};
}
