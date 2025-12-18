use crate::components::windows::{
  AboutButton, MoreInfoButton, PictureButton, SettingsButton, UpdatesButton, WindowParentProps,
};
use crate::components::{
  MoreInfoState, PlayerState, RadioApi, RadioAudio, RadioState, SettingsState, Visualizer,
  WindowTemplate, VolumeSlider, format_time, get_api_url,
};

use dioxus::prelude::*;
use dioxus_sdk::time::use_interval;
use std::time::Duration;

// Top menu bar of the player
#[component]
pub fn PlayerMenu() -> Element {
  let download_link = use_context::<RadioState>().download_link;

  rsx! {
    div {
      id: "player-menu",
      class: "menu-bar",
      div {
        class: "action",
        a {
          id: "home-button",
          href: "https://github.com/rpgwaiter/basedradio-rs",
          target: "_blank",
          role: "button",
          "Home"
        },
      },
      // div {
      //   class: "action",
      //   AboutButton {  }
      // },
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

// The info about the current song
#[component]
pub fn PlayerStats(system: Signal<String>, track: Signal<String>, game: Signal<String>) -> Element {
  rsx! {
    div {
      class: "player-stats content",
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

// Holds most of the content and other elements,
// also in charge of pulling from the api
#[component]
pub fn PlayerContent() -> Element {
  let mut player_state = use_context::<PlayerState>();
  let mut radio_state = use_context::<RadioState>();
  let mut elapsed = player_state.elapsed;
  let mut duration = player_state.duration;
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
      player_state.total_songs.set(response.status.total_songs);
      more.more_info.set(response.more_info)
    }
  };

  // Initial load
  // This if ensures that we don't spam the api
  if player_state.title.peek().as_str() == "Loading info..." {
    spawn(fetch_info());
  };

  // TODO: this spams connections if the api is dead
  use_interval(Duration::from_secs(1), move |_| {
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
        PictureButton { }
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
        SettingsButton { },
        VolumeSlider { }
      }
    }
  }
}



#[component]
pub fn Player(props: WindowParentProps) -> Element {
  let player_state = use_context::<PlayerState>();
  let bounce = use_context::<SettingsState>().bounce;

  rsx! {
    WindowTemplate {
      title: "BasedRadio",
      id: "based-radio",
      header_icon: true,
      footer_text: Some(format!("Listeners: {:?} | Total Songs: {:?}", (player_state.listeners)(), (player_state.total_songs)())),
      bounce: Some(bounce),
      index: 1,
      is_visible: props.is_visible,
      extra_menu_btn: Some(rsx!{ AboutButton {} }),
      PlayerMenu { },
      div {
        id: "player-container",
        class: "minimizable content",
        PlayerContent { }
      }
    },
  }
}
