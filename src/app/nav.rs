// SPDX-License-Identifier: MIT
//! The nav bar.

use super::Page;
use crate::fl;
use cosmic::widget::{icon, nav_bar::Model};

/// Initializes a custom nav model.
pub fn init_model() -> Model {
  let mut nav = Model::default();

  nav
    .insert()
    .text(fl!("page-home"))
    .data::<Page>(Page::Home)
    .icon(icon::from_name("user-home-symbolic"))
    .activate();

  nav
    .insert()
    .text(fl!("page-config"))
    .data::<Page>(Page::Config)
    .icon(icon::from_name("applications-system-symbolic"));

  nav
}
