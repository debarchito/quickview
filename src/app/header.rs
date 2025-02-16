// SPDX-License-Identifier: MIT
//! The header bar.

use super::{Action, AppModel, Message};
use crate::fl;
use cosmic::{widget::menu, Element};

/// Attaches elements to the start section of the header.
pub fn attach_to_start(model: &AppModel) -> Vec<Element<Message>> {
  let menu_bar = menu::bar(vec![menu::Tree::with_children(
    menu::root(fl!("view")),
    menu::items(
      &model.key_binds,
      vec![menu::Item::Button(
        fl!("view-about"),
        None,
        Action::MenuAbout,
      )],
    ),
  )]);

  vec![menu_bar.into()]
}
