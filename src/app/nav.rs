// SPDX-License-Identifier: MIT
//! The nav bar.

use crate::{
  app::pages::views::{config::Config, home::Home, ViewPage},
  fl,
};
use cosmic::widget::{icon, nav_bar::Model};

/// Initializes a custom nav model.
pub fn init_model() -> Model {
  let mut nav = Model::default();

  nav
    .insert()
    .text(fl!("pages-views-home"))
    .data(Box::new(Home) as Box<dyn ViewPage>)
    .icon(icon::from_name("user-home-symbolic"))
    .activate();

  nav
    .insert()
    .text(fl!("pages-views-config"))
    .data(Box::new(Config) as Box<dyn ViewPage>)
    .icon(icon::from_name("applications-system-symbolic"));

  nav
}
