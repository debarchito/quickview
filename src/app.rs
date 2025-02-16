// SPDX-License-Identifier: MIT
//! Orchestrates the app.

use crate::config::Config;
use crate::fl;
use cosmic::app::{context_drawer, Core, Task};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::Subscription;
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::{self, menu, nav_bar};
use cosmic::{Application, ApplicationExt, Element};
use futures_util::SinkExt;
use std::collections::HashMap;
mod context_page;
mod header;
mod nav;
mod view_page;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

/// The application model.
pub struct AppModel {
  core: Core,
  context_page: ContextPage,
  nav_model: nav_bar::Model,
  key_binds: HashMap<menu::KeyBind, Action>,
  config: Config,
}

/// The page to display in the main view.
pub enum Page {
  Home,
  Config,
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
  #[default]
  None,
  About,
}

/// The messages that can be sent to the application.
#[derive(Debug, Clone)]
pub enum Message {
  ToggleContextPage(ContextPage),
  SubscriptionChannel,
  UpdateConfig(Config),
  OpenUrl(String),
}

/// The actions that can be taken in the application.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
  // Menu actions
  MenuAbout,
}

impl Application for AppModel {
  type Executor = cosmic::executor::Default;
  type Flags = ();
  type Message = Message;

  const APP_ID: &'static str = "dev.debarchito.quickview";

  /// Grants access to the COSMIC core.
  /// @required_by_trait
  fn core(&self) -> &Core {
    &self.core
  }

  /// Grants mutable access to the COSMIC core.
  /// @required_by_trait
  fn core_mut(&mut self) -> &mut Core {
    &mut self.core
  }

  /// Creates the application, and optionally emits task on initialize.
  /// @required_by_trait
  fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
    let mut app = AppModel {
      core,
      context_page: ContextPage::default(),
      nav_model: nav::init_model(),
      key_binds: HashMap::new(),
      config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
        .map(|context| match Config::get_entry(&context) {
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
  /// @overridden
  fn header_start(&self) -> Vec<Element<Self::Message>> {
    header::attach_to_start(self)
  }

  /// Integrate with the custom nav model.
  /// @overridden
  fn nav_model(&self) -> Option<&nav_bar::Model> {
    Some(&self.nav_model)
  }

  /// Display a context drawer if the context page is requested.
  /// @overridden
  fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
    if !self.core.window.show_context {
      return None;
    }

    Some(match self.context_page {
      ContextPage::About => context_drawer::context_drawer(
        context_page::about::init(),
        Message::ToggleContextPage(ContextPage::About),
      )
      .title(fl!("view-about")),
      _ => return None,
    })
  }

  /// Constructs the actual viewable content.
  /// @required_by_trait
  fn view(&self) -> Element<Self::Message> {
    let id = self.nav_model.active();
    let page = self.nav_model.data::<Page>(id);

    match page {
      Some(Page::Home) => view_page::home::init(),
      Some(Page::Config) => view_page::config::init(),
      // TODO: Something went wrong, show an error page.
      None => widget::container("No page found").into(),
    }
  }

  /// Event sources that are to be listened to.
  /// @overridden
  fn subscription(&self) -> Subscription<Self::Message> {
    struct Subscriptions;

    Subscription::batch(vec![
      Subscription::run_with_id(
        std::any::TypeId::of::<Subscriptions>(),
        cosmic::iced::stream::channel(4, move |mut channel| async move {
          _ = channel.send(Message::SubscriptionChannel).await;
          futures_util::future::pending().await
        }),
      ),
      self
        .core()
        .watch_config::<Config>(Self::APP_ID)
        .map(|update| {
          for why in update.errors {
            tracing::error!(?why, "app config error");
          }
          Message::UpdateConfig(update.config)
        }),
    ])
  }

  /// Respond to an application-specific message.
  /// @overridden
  fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
    match message {
      Message::ToggleContextPage(context_page) => {
        if self.context_page == context_page {
          self.core.window.show_context = !self.core.window.show_context;
        } else {
          self.context_page = context_page;
          self.core.window.show_context = true;
        }
      }
      Message::SubscriptionChannel => {
        tracing::info!("subscription channel message");
      }
      Message::UpdateConfig(config) => {
        self.config = config;
      }
      Message::OpenUrl(url) => match open::that_detached(&url) {
        Ok(()) => {}
        Err(err) => {
          eprintln!("failed to open {url:?}: {err}");
        }
      },
    }
    Task::none()
  }

  /// Called when a nav item is selected.
  /// @overridden
  fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
    self.nav_model.activate(id);
    self.update_title()
  }
}

impl AppModel {
  /// Updates the header and window titles.
  pub fn update_title(&mut self) -> Task<Message> {
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

impl MenuAction for Action {
  type Message = Message;

  /// Converts the menu action to a message.
  /// @required_by_trait
  fn message(&self) -> Self::Message {
    match self {
      Action::MenuAbout => Message::ToggleContextPage(ContextPage::About),
    }
  }
}
