// SPDX-License-Identifier: MIT
//! The home page.

use super::super::Message;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::Apply;
use cosmic::{widget, Element};

/// Initializes the home page.
pub fn init<'a>() -> Element<'a, Message> {
  widget::text::title1("This is the home page!")
    .apply(widget::container)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}
