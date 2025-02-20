// SPDX-License-Identifier: MIT
//! The about context page.

use super::ContextPage;
use crate::{
  app::{AppMessage, APP_ICON, REPOSITORY},
  fl,
};
use cosmic::{cosmic_theme, iced::Alignment, theme, widget, Element};

#[derive(Default)]
pub struct About;

impl ContextPage for About {
  fn view(&self) -> Element<AppMessage> {
    let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

    let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));
    let title = widget::text::title3(fl!("app-title"));

    let hash = env!("VERGEN_GIT_SHA");
    let short_hash: String = hash.chars().take(7).collect();

    let link = widget::button::link(REPOSITORY)
      .on_press(AppMessage::OpenUrl(REPOSITORY.into()))
      .padding(0);

    widget::column()
      .push(icon)
      .push(title)
      .push(link)
      .push(
        widget::button::link(fl!("git-description", hash = short_hash.as_str()))
          .on_press(AppMessage::OpenUrl(format!("{REPOSITORY}/commits/{hash}")))
          .padding(0),
      )
      .align_x(Alignment::Center)
      .spacing(space_xxs)
      .into()
  }
}
