// SPDX-License-Identifier: MIT
//! Orchestrates the app.

mod header;
mod nav;
mod pages;
use crate::{core::config::QuickviewConfig, fl};
use cosmic::{
  app::{context_drawer, Core, Task},
  cosmic_config::{self, CosmicConfigEntry},
  iced::Subscription,
  widget::{self, menu, menu::action::MenuAction, nav_bar},
  Application, ApplicationExt, Element,
};
use pages::contexts::{self, ContextPage};
use pages::views;
use std::collections::HashMap;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

/// The application model.
pub struct App {
  core: Core,
  context_page: contexts::Context,
  nav_model: nav_bar::Model,
  key_binds: HashMap<menu::KeyBind, AppAction>,
  config: QuickviewConfig,
}

/// The messages that can be sent to the application.
#[derive(Debug, Clone)]
pub enum AppMessage {
  ToggleContextPage(contexts::Context),
  UpdateConfig(QuickviewConfig),
  OpenUrl(String),
}

/// The actions that can be taken in the application.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppAction {
  // Menu actions
  MenuToggleAboutContextPage,
}

impl Application for App {
  type Executor = cosmic::executor::Default;
  type Flags = ();
  type Message = AppMessage;

  const APP_ID: &'static str = "io.github.debarchito.quickview";

  /// Grants access to the COSMIC core.
  fn core(&self) -> &Core {
    &self.core
  }

  /// Grants mutable access to the COSMIC core.
  fn core_mut(&mut self) -> &mut Core {
    &mut self.core
  }

  /// Creates the application, and optionally emits task on initialize.
  fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
    let mut app = App {
      core,
      context_page: contexts::Context::default(),
      nav_model: nav::init_model(),
      key_binds: HashMap::new(),
      config: cosmic_config::Config::new(Self::APP_ID, QuickviewConfig::VERSION)
        .map(|context| match QuickviewConfig::get_entry(&context) {
          Ok(config) => config,
          Err((errors, config)) => {
            for why in errors {
              tracing::error!(%why, "error loading app config");
            }
            config
          }
        })
        .unwrap_or_default(),
    };

    let task = app.update_title();

    (app, task)
  }

  /// Attaches elements to the start section of the header.
  fn header_start(&self) -> Vec<Element<Self::Message>> {
    header::attach_to_start(self)
  }

  /// Integrate with the custom nav model.
  fn nav_model(&self) -> Option<&nav_bar::Model> {
    Some(&self.nav_model)
  }

  /// Called when a nav item is selected.
  fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
    self.nav_model.activate(id);
    self.update_title()
  }

  /// Display a context drawer if the context page is requested.
  fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
    use contexts::{about, Context};

    if !self.core.window.show_context {
      return None;
    }

    match self.context_page {
      Context::About => Some(
        context_drawer::context_drawer(
          about::About.view(),
          AppMessage::ToggleContextPage(Context::About),
        )
        .title(fl!("pages-context-view-about")),
      ),
      _ => None,
    }
  }

  /// Constructs the actual viewable content.
  fn view(&self) -> Element<Self::Message> {
    if let Some(page) = self.nav_model.active_data::<Box<dyn views::ViewPage>>() {
      page.view()
    } else {
      widget::text("Unknown Page!").into()
    }
  }

  /// Event sources that are to be listened to.
  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::batch(vec![self
      .core()
      .watch_config::<QuickviewConfig>(Self::APP_ID)
      .map(|update| {
        for why in update.errors {
          tracing::error!(?why, "app config error");
        }
        AppMessage::UpdateConfig(update.config)
      })])
  }

  /// Respond to an application-specific message.
  fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
    let mut tasks = Vec::new();

    match message.clone() {
      AppMessage::ToggleContextPage(context_page) => {
        if self.context_page == context_page {
          self.core.window.show_context = !self.core.window.show_context;
        } else {
          self.context_page = context_page;
          self.core.window.show_context = true;
        }
      }
      AppMessage::UpdateConfig(config) => {
        self.config = config;
      }
      AppMessage::OpenUrl(url) => {
        if let Err(why) = open::that_detached(&url) {
          tracing::error!(%why, "error opening URL");
        }
      }
    }

    let entities = self
      .nav_model
      .iter()
      .collect::<Vec<widget::segmented_button::Entity>>();

    for entity in entities {
      let page = self.nav_model.data_mut::<Box<dyn views::ViewPage>>(entity);
      if let Some(page) = page {
        tasks.push(page.update(message.clone()));
      }
    }

    Task::batch(tasks)
  }
}

impl App {
  /// Updates the header and window titles.
  pub fn update_title(&mut self) -> Task<AppMessage> {
    let mut window_title = fl!("app-title");

    if let Some(page) = self.nav_model.text(self.nav_model.active()) {
      window_title.push_str(" — ");
      window_title.push_str(page);
    }

    if let Some(id) = self.core.main_window_id() {
      self.set_window_title(window_title, id)
    } else {
      Task::none()
    }
  }
}

impl MenuAction for AppAction {
  type Message = AppMessage;

  /// Converts the menu action to a message.
  fn message(&self) -> Self::Message {
    match self {
      AppAction::MenuToggleAboutContextPage => {
        AppMessage::ToggleContextPage(contexts::Context::About)
      }
    }
  }
}
