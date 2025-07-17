use crate::components::{PlayerState, RadioApi, RadioState, Visualizer, Window, audio::RadioAudio};
use dioxus::prelude::*;
use urlencoding::encode;

use dioxus_sdk::utils::timing::use_interval;
use std::time::Duration;

pub static STREAM_MP3: &str = "https://cast.based.radio/vgm.mp3";
pub static API_URL: &str = "https://api.based.radio";

// TODO: move to a lib
fn add_zeros(e: i16, t: usize) -> String {
  let s = e.to_string();
  format!("{:0>width$}", s, width = t)
}

fn format_time(e: i16) -> String {
  let e = e % 3600; // seconds within the hour
  let min = add_zeros(e / 60, 2);
  let sec = add_zeros(e % 60, 2);
  format!("{}:{}", min, sec)
}

#[component]
pub fn PlayerMenu() -> Element {
  let mut downloadLink = use_context::<RadioState>().downloadLink;
  let mut aboutIsVisible = use_context::<RadioState>().aboutIsVisible;
  rsx! {
      div {
          id: "player-menu",
          class: "menu-bar",
          div {
              class: "action",
              a {
                  id: "home-button",
                  href: "/",
                  role: "button",
                  "Home"
              },
          },
          div {
              class: "action",
              a {
                  onclick: move |event| aboutIsVisible.set(!aboutIsVisible()),
                  id: "about-show",
                  role: "button",
                  "About",

              }
          },
          div {
              class: "action",
              a {
                  id: "download-btn",
                  role: "button",
                  href: downloadLink,
                  download: downloadLink().rsplit_once('/').unwrap().1,
                  target: "_blank",
                  "Download"
              }
          },
          div {
              class: "action",
              style: "float: right;",
              a {
                  id: "updates-show",
                  role: "button",
                  "Updates"
              }
          }
      }
  }
}

#[component]
pub fn PlayerStats(system: Signal<String>, track: Signal<String>, game: Signal<String>) -> Element {
  rsx! {
      div {
          class: "player-stats",
          div {
              class: "player-game",
              strong { "Game: " }, a { id: "current-game", "{game}" }
          },
          div {
              class: "player-track",
              strong { "Track: " }, a { id: "current-track", "{track}" }
          },
          div {
              class: "player-system",
              strong { "System: " }, a { id: "current-system", "{system}" }
          }
      },
      div {

      }
  }
}

#[component]
pub fn PlayerContent() -> Element {
  let mut elapsed = use_context::<PlayerState>().elapsed;
  let mut duration = use_context::<PlayerState>().duration;
  let mut game = use_context::<PlayerState>().game;
  let mut track = use_context::<PlayerState>().title;
  let mut system = use_context::<PlayerState>().system;
  let mut cover_art = use_context::<PlayerState>().cover;
  let mut downloadLink = use_context::<RadioState>().downloadLink;

  let fetch_info = move || async move {
    if let Ok(response) = reqwest::get(API_URL)
      .await
      .unwrap()
      .json::<RadioApi>()
      .await
    {
      game.set(response.song.game);
      track.set(response.song.title);
      system.set(response.song.system);
      cover_art.set(response.song.cover);
      // There just has to be a better way
      elapsed.set(response.status.elapsed.parse::<f32>().unwrap().round() as i16);
      duration.set(response.status.duration.parse::<f32>().unwrap().round() as i16);
      downloadLink.set(format!(
        "https://files.based.radio/{}",
        encode(&response.song.file).to_string()
      )); // TODO: grab link url from env or something
    }
  };

  // Initial load
  // This if ensures that we don't spam the api
  // TODO: track this better. If the api is dead it will get spammed
  if track.peek().as_str() == "" {
    print!("laoding thing");
    spawn(fetch_info());
  };

  use_interval(Duration::from_secs(1), move || {
    if elapsed() >= duration() {
      spawn(fetch_info());
    };
    elapsed += 1
  });

  rsx! {
      div {
          class: "stream-meta",
          div {
              class: "player-cover-art",
              img { id: "current-cover", src: "{cover_art}", alt: "Cover Art", style: "margin: auto; display: block;" }
          },
          PlayerStats { game: game, system: system, track: track  }
      },
      div {
          class: "player-meta",
          Visualizer { },
          div {
              class: "player-time-container text-field",
              div {
                  id: "player-time",
                  "~~~ ",
                  a { id: "elapsed-time", "{format_time(*elapsed.read())}" } " / " a { id: "song-duration", "{format_time(*duration.read())}"}
                  " ~~~"
              }
          },
          RadioAudio {  }
      }
  }
}

#[component]
pub fn Player() -> Element {
  let playerState = use_context_provider(|| PlayerState::new());
  rsx! {
      div {
          id: "window-player",
          class: "win98",
          style: "z-index: 0 !important;",
          Window {
              title: "BasedRadio",
              id: "based-radio",
              header_icon: true,
              PlayerMenu { },
                  div {
                      id: "player-container",
                      class: "minimizable content",
                      PlayerContent { }
                  }

          },

      }
  }
}
