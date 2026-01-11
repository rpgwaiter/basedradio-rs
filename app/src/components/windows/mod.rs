use dioxus::prelude::*;

mod about;
mod moreinfo;
mod news;
mod picture_viewer;
mod player;
mod settings;
mod updates;

#[derive(PartialEq, Props, Clone)]
pub struct WindowParentProps {
  pub is_visible: Signal<bool>,
}

pub use player::Player;

pub use about::{AboutButton, AboutWindow};
pub use moreinfo::{MoreInfoButton, MoreInfoWindow};
pub use news::{NewsWindow, news_loader};
pub use picture_viewer::{PictureButton, PictureWindow};
pub use settings::{SettingsButton, SettingsState, SettingsWindow};
pub use updates::{UpdatesButton, UpdatesWindow};
