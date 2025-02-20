// SPDX-License-Identifier: MIT
//! The entry point.

mod app;
mod core;
use core::settings;

fn main() -> cosmic::iced::Result {
  settings::init();
  cosmic::app::run::<app::App>(settings::settings(), ())
}
