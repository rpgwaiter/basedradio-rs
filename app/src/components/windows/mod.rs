use dioxus::prelude::*;

mod about;
mod moreinfo;
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
pub use picture_viewer::{PictureButton, PictureWindow};
pub use settings::{SettingsButton, SettingsWindow};
pub use updates::{UpdatesButton, UpdatesWindow};
