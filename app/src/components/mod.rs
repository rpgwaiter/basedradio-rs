use crate::components::windows::Player;
use dioxus::prelude::*;
use std::env;

pub mod windows;

mod audio;
pub use audio::RadioAudio;

mod window_template;
pub use window_template::WindowTemplate;

mod visualizer;
pub use visualizer::Visualizer;

mod taskbar;
pub use taskbar::{Taskbar, TaskbarItem, TaskbarItemProps};

pub fn get_stream_mp3() -> String {
  env::var("STREAM_MP3").unwrap_or("https://cast.based.radio/vgm.mp3".into())
}

pub fn get_api_url() -> String {
  env::var("API_URL").unwrap_or("https://api.based.radio".into())
}

pub static ICON_FAVICON: Asset = asset!("/assets/icons/favicon-32x32.png");

// TODO: move to a lib
pub fn add_zeros(e: i16, t: usize) -> String {
  let s = e.to_string();
  format!("{:0>width$}", s, width = t)
}

pub fn format_time(e: i16) -> String {
  let e = e % 3600; // seconds within the hour
  let min = add_zeros(e / 60, 2);
  let sec = add_zeros(e % 60, 2);
  format!("{}:{}", min, sec)
}

#[derive(serde::Deserialize, Debug)]
pub struct Song {
  background: Option<String>,
  cover: String,
  game: String,
  system: String,
  title: String,
  download_link: String,
}

// TODO: fix these data types
#[derive(serde::Deserialize, Debug)]
pub struct Status {
  elapsed: i16,
  duration: i16,
  listeners: i16,
  total_songs: u32,
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
  pub total_songs: Signal<u32>,
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
      total_songs: Signal::new(0 as u32),
    }
  }
}

#[derive(Clone, Copy)]
pub struct SettingsState {
  pub use_background: Signal<bool>,
  pub bounce: Signal<bool>,
}

impl SettingsState {
  pub fn new() -> Self {
    SettingsState {
      use_background: Signal::new(true),
      bounce: Signal::new(false),
    }
  }
}

#[derive(Clone, Copy)]
pub struct DragState {
  pub active_window: Signal<String>,
  pub is_dragging: Signal<bool>,
  pub dim_x: Signal<f64>,
  pub dim_y: Signal<f64>,
  pub previous_x: Signal<f64>,
  pub previous_y: Signal<f64>,
}
impl DragState {
  pub fn new() -> Self {
    DragState {
      active_window: Signal::new(String::from("based-radio")),
      is_dragging: Signal::new(false),
      dim_x: Signal::new(0 as f64),
      dim_y: Signal::new(0 as f64),
      previous_x: Signal::new(0 as f64),
      previous_y: Signal::new(0 as f64),
    }
  }
}

#[derive(Clone, Copy)]
pub struct Visibility {
  pub about: Signal<bool>,
  pub settings: Signal<bool>,
  pub updates: Signal<bool>,
  pub more_info: Signal<bool>,
  pub picture: Signal<bool>,
}

impl Visibility {
  pub fn new() -> Self {
    Visibility {
      about: Signal::new(false),
      settings: Signal::new(false),
      updates: Signal::new(false),
      more_info: Signal::new(false),
      picture: Signal::new(false),
    }
  }
}

// TODO: add basically all state here
#[derive(Clone, Copy)]
pub struct RadioState {
  pub download_link: Signal<String>,
  pub updates: Signal<Vec<String>>,
  pub drag_state: DragState,
  pub open_windows: Signal<Vec<OpenWindow>>,
}

impl RadioState {
  pub fn new() -> Self {
    RadioState {
      download_link: Signal::new(String::from("/")),
      updates: Signal::new(vec![String::from("Loading updates...")]),
      drag_state: DragState::new(),
      open_windows: Signal::new(vec![OpenWindow {
        id: "based-radio".to_string(),
        window: rsx! { Player {  } },
        taskbar_item: rsx! { TaskbarItem { id: "based-radio", icon: None, title: "BasedRadio", is_visible: Signal::new(true) }},
      }]),
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

#[derive(Clone)]
pub struct OpenWindow {
  pub id: String,
  pub taskbar_item: Element,
  pub window: Element,
}
