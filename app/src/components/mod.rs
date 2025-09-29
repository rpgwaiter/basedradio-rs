use dioxus::prelude::Signal;

pub static STREAM_MP3: &str = "https://cast.based.radio/vgm.mp3";
pub static API_URL: &str = "https://api.based.radio";

pub mod player;
pub use player::Player;

pub mod about;
pub use about::About;

pub mod updates;
pub use updates::Updates;

pub mod moreinfo;
pub use moreinfo::{MoreInfo, MoreInfoButton};

pub mod audio;

pub mod window;
pub use window::Window;

pub mod visualizer;
pub use visualizer::Visualizer;

#[derive(serde::Deserialize)]
pub struct Song {
  album: String,
  artist: String,
  file: String,
  duration: String, // Eventually will be a number
  game: String,
  system: String,
  title: String,
  cover: String,
}

#[derive(serde::Deserialize)]
pub struct Status {
  elapsed: String,
  duration: String,
}

#[derive(serde::Deserialize)]
pub struct RadioApi {
  song: Song,
  status: Status,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
  album: Signal<String>,
  artist: Signal<String>,
  file: Signal<String>,
  duration: Signal<i16>, // Eventually will be a number
  elapsed: Signal<i16>,
  game: Signal<String>,
  system: Signal<String>,
  title: Signal<String>,
  cover: Signal<String>,
}

impl PlayerState {
  pub fn new() -> Self {
    PlayerState {
      album: Signal::new("".to_string()),
      artist: Signal::new("".to_string()),
      file: Signal::new("".to_string()),
      duration: Signal::new(0 as i16), // Eventually will be a number
      elapsed: Signal::new(0 as i16),
      game: Signal::new("".to_string()),
      system: Signal::new("".to_string()),
      title: Signal::new("Loading info...".to_string()),
      cover: Signal::new("".to_string()),
    }
  }
}

// TODO: add basically all state here
#[derive(Clone, Copy)]
pub struct RadioState {
  aboutIsVisible: Signal<bool>,
  updatesIsVisible: Signal<bool>,
  moreInfoIsVisible: Signal<bool>,
  downloadLink: Signal<String>,
}

impl RadioState {
  pub fn new() -> Self {
    RadioState {
      aboutIsVisible: Signal::new(false),
      updatesIsVisible: Signal::new(false),
      moreInfoIsVisible: Signal::new(false),
      downloadLink: Signal::new("/".to_string()),
    }
  }
}
