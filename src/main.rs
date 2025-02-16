// SPDX-License-Identifier: MIT

mod app;
mod config;
mod i18n;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> cosmic::iced::Result {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(if cfg!(debug_assertions) {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
  i18n::init(&requested_languages);

  let settings = cosmic::app::Settings::default().size_limits(
    cosmic::iced::Limits::NONE
      .min_width(360.0)
      .min_height(180.0),
  );

  cosmic::app::run::<app::AppModel>(settings, ())
}
