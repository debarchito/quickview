// SPDX-License-Identifier: MIT
//! The "about" context page.

use super::super::{Message, APP_ICON, REPOSITORY};
use crate::fl;
use cosmic::{cosmic_theme, iced::Alignment, theme, widget, Element};

/// Initializes the "about" context page.
pub fn init<'a>() -> Element<'a, Message> {
  let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

  let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));
  let title = widget::text::title3(fl!("app-title"));

  let hash = env!("VERGEN_GIT_SHA");
  let short_hash: String = hash.chars().take(7).collect();

  let link = widget::button::link(REPOSITORY)
    .on_press(Message::OpenUrl(REPOSITORY.into()))
    .padding(0);

  widget::column()
    .push(icon)
    .push(title)
    .push(link)
    .push(
      widget::button::link(fl!("git-description", hash = short_hash.as_str()))
        .on_press(Message::OpenUrl(format!("{REPOSITORY}/commits/{hash}")))
        .padding(0),
    )
    .align_x(Alignment::Center)
    .spacing(space_xxs)
    .into()
}
