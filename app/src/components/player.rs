use crate::components::{
  MoreInfoButton, MoreInfoState, PlayerState, RadioApi, RadioState, SettingsButton, Visualizer,
  Window, audio::RadioAudio, get_api_url, updates::UpdatesButton,
};
use dioxus::prelude::*;

use dioxus_sdk::utils::timing::use_interval;
use std::time::Duration;

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
  let mut download_link = use_context::<RadioState>().download_link;
  let mut about_is_visible = use_context::<RadioState>().about_is_visible;
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
          onclick: move |event| about_is_visible.toggle(),
          id: "about-show",
          role: "button",
          "About"
        }
      },
      div {
        class: "action",
        a {
          id: "download-btn",
          role: "button",
          href: download_link,
          download: download_link().rsplit_once('/').unwrap().1,
          target: "_blank",
          "Download"
        }
      },
      div {
        class: "action",
        style: "float: right;",
        UpdatesButton { }
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
    }
  }
}

#[component]
pub fn PlayerContent() -> Element {
  let mut player_state = use_context::<PlayerState>();
  let mut radio_state = use_context::<RadioState>();
  let mut elapsed = use_context::<PlayerState>().elapsed;
  let mut duration = use_context::<PlayerState>().duration;
  let mut more = use_context::<MoreInfoState>();

  let fetch_info = move || async move {
    if let Ok(response) = reqwest::get(get_api_url())
      .await
      .unwrap()
      .json::<RadioApi>()
      .await
    {
      player_state.game.set(response.song.game);
      player_state.title.set(response.song.title);
      player_state.system.set(response.song.system);
      player_state.cover.set(response.song.cover);
      player_state.background.set(response.song.background);
      elapsed.set(response.status.elapsed);
      duration.set(response.status.duration);
      radio_state.download_link.set(response.song.download_link);
      player_state.listeners.set(response.status.listeners);
      more.more_info.set(response.more_info)
    }
  };

  // Initial load
  // This if ensures that we don't spam the api
  if player_state.title.peek().as_str() == "Loading info..." {
    spawn(fetch_info());
  };

  // TODO: this spams connections if the api is dead
  use_interval(Duration::from_secs(1), move || {
    if elapsed() >= duration() {
      spawn(fetch_info());
    };
    elapsed += 1
  });

  rsx! {
    document::Title { "{player_state.title} | BasedRadio" }
    div {
      class: "stream-meta",
      div {
        class: "player-cover-art",
        img { id: "current-cover", src: "{player_state.cover}", alt: "Cover Art", style: "margin: auto; display: block;" }
      },
      PlayerStats { game: player_state.game, system: player_state.system, track: player_state.title  }
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
      div {
      class: "content-buttons",
        RadioAudio { },
        MoreInfoButton { },
        SettingsButton { }
      }
    }
  }
}

#[component]
pub fn Player() -> Element {
  let listeners = use_context::<PlayerState>().listeners;

  rsx! {
    div {
      id: "window-player",
      class: "win98",
      style: "z-index: 0 !important;",
      Window {
        title: "BasedRadio",
        id: "based-radio",
        header_icon: true,
        footer_text: Some(format!("Listeners: {:?}", listeners())),
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
