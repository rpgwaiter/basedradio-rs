use dioxus::prelude::Signal;
use std::env;

pub fn get_stream_mp3() -> String {
  env::var("STREAM_MP3").unwrap_or("https://cast.based.radio/vgm.mp3".into())
}

pub fn get_api_url() -> String {
  env::var("API_URL").unwrap_or("https://api.based.radio".into())
}

pub mod windows;

mod audio;
pub use audio::RadioAudio;

mod window_template;
pub use window_template::WindowTemplate;

mod visualizer;
pub use visualizer::Visualizer;

#[derive(serde::Deserialize, Debug)]
pub struct Song {
  background: Option<String>,
  cover: String,
  game: String,
  system: String,
  title: String,
  download_link: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Status {
  elapsed: i16,
  duration: i16,
  listeners: i16,
}

#[derive(serde::Deserialize)]
pub struct RadioApi {
  song: Song,
  status: Status,
  more_info: UpstreamMoreInfo,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
  pub duration: Signal<i16>, // Eventually will be a number
  pub elapsed: Signal<i16>,
  pub game: Signal<String>,
  pub system: Signal<String>,
  pub title: Signal<String>,
  pub cover: Signal<String>,
  pub background: Signal<Option<String>>,
  pub listeners: Signal<i16>,
}

impl PlayerState {
  pub fn new() -> Self {
    PlayerState {
      duration: Signal::new(0 as i16), // Eventually will be a number
      elapsed: Signal::new(0 as i16),
      game: Signal::new("".to_string()),
      system: Signal::new("".to_string()),
      title: Signal::new("Loading info...".to_string()),
      cover: Signal::new("".to_string()),
      background: Signal::new(None as Option<String>),
      listeners: Signal::new(0 as i16),
    }
  }
}

#[derive(Clone, Copy)]
pub struct SettingsState {
  pub use_background: Signal<bool>,
}

impl SettingsState {
  pub fn new() -> Self {
    SettingsState {
      use_background: Signal::new(true),
    }
  }
}

// TODO: add basically all state here
#[derive(Clone, Copy)]
pub struct RadioState {
  about_is_visible: Signal<bool>,
  settings_is_visible: Signal<bool>,
  updates_is_visible: Signal<bool>,
  more_info_is_visible: Signal<bool>,
  download_link: Signal<String>,
  updates: Signal<Vec<String>>,
}

impl RadioState {
  pub fn new() -> Self {
    RadioState {
      about_is_visible: Signal::new(false),
      settings_is_visible: Signal::new(false),
      updates_is_visible: Signal::new(false),
      more_info_is_visible: Signal::new(false),
      download_link: Signal::new("/".to_string()),
      updates: Signal::new(vec![String::from("Loading updates...")]),
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UpstreamMoreInfo {
  pub game: Option<TitleLangs>,
  pub links: Option<InfoSites>,
  pub notes: Vec<String>,
}

#[derive(Clone)]
pub struct MoreInfoState {
  pub more_info: Signal<UpstreamMoreInfo>,
}

impl MoreInfoState {
  pub fn new() -> MoreInfoState {
    MoreInfoState {
      more_info: Signal::new(UpstreamMoreInfo::new()),
    }
  }
}

impl UpstreamMoreInfo {
  pub fn new() -> UpstreamMoreInfo {
    let mut notes: Vec<String> = Vec::new();
    notes.push(format!("You should never see this"));
    UpstreamMoreInfo {
      game: None,
      links: None,
      notes: notes,
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct InfoSites {
  pub wikipedia: Option<String>,
  pub khinsider: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TitleLangs {
  pub en: Option<String>,
  pub ja: Option<String>,
}
