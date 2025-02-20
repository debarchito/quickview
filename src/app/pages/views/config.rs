// SPDX-License-Identifier: MIT
//! The config view page.

use super::ViewPage;
use crate::app::AppMessage;
use cosmic::{
  iced::{
    alignment::{Horizontal, Vertical},
    Length,
  },
  widget, Apply, Element,
};

#[derive(Default)]
pub struct Config;

impl ViewPage for Config {
  fn view(&self) -> Element<AppMessage> {
    widget::text::title1("This is the config page!")
      .apply(widget::container)
      .width(Length::Fill)
      .height(Length::Fill)
      .align_x(Horizontal::Center)
      .align_y(Vertical::Center)
      .into()
  }
}
