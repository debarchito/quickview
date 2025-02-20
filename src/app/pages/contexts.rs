// SPDX-License-Identifier: MIT
//! The context pages.

pub mod about;
use crate::app::AppMessage;
use cosmic::Element;

/// Represents the structure of a context page. All context pages must implement this trait.
pub trait ContextPage {
  /// Constructs the actual viewable content.
  fn view(&self) -> Element<AppMessage>;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Context {
  #[default]
  None,
  About,
}
