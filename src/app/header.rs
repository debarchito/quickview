// SPDX-License-Identifier: MIT
//! The header bar.

use super::{App, AppAction, AppMessage};
use crate::fl;
use cosmic::{widget::menu, Element};

/// Attaches elements to the start section of the header.
pub fn attach_to_start(model: &App) -> Vec<Element<AppMessage>> {
  let menu_bar = menu::bar(vec![menu::Tree::with_children(
    menu::root(fl!("pages-context-view")),
    menu::items(
      &model.key_binds,
      vec![menu::Item::Button(
        fl!("pages-context-view-about"),
        None,
        AppAction::MenuToggleAboutContextPage,
      )],
    ),
  )]);

  vec![menu_bar.into()]
}
