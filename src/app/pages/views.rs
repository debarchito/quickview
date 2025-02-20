// SPDX-License-Identifier: MIT
//! The view pages.

pub mod config;
pub mod home;
use crate::app::AppMessage;
use cosmic::{app::Message, Element, Task};

/// Represents the structure of a view page. All view pages must implement this trait.
pub trait ViewPage {
  /// Constructs the actual viewable content.
  #[allow(unused)]
  fn view(&self) -> Element<AppMessage>;
  /// Respond to a view-specific message.
  #[allow(unused)]
  fn update(&mut self, message: AppMessage) -> Task<Message<AppMessage>> {
    Task::none()
  }
}
