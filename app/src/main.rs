mod components;

use components::windows::Player;
use components::{
  MoreInfoState, OpenWindow, PlayerState, RadioState, SettingsState, Taskbar, TaskbarItem,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use crate::components::play_sound_effect;
use crate::components::windows::news_loader;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// TODO: unify and clean these up
const BASED_RADIO_CSS: Asset = asset!("/assets/style/based98.css");
const APP_CSS: Asset = asset!("/assets/style/app.css");

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: APP_CSS }
      document::Link { rel: "stylesheet", href: BASED_RADIO_CSS }
      Router::<Route> {}
  }
}

/// Home page
#[component]
fn Home() -> Element {
  let mouseupsound = asset!("/assets/sounds/mousedown.mp3");
  let mousedownsound = asset!("/assets/sounds/mouseup.mp3");

  let radio_state = use_context_provider(|| RadioState::new());
  use_context_provider(|| MoreInfoState::new());
  let settings_state = use_context_provider(|| SettingsState::new());
  let player_state = use_context_provider(|| PlayerState::new());
  let mut open_windows = radio_state.open_windows;
  let player_is_visible = Signal::new(true);

  // set this to false to disable the news window
  let news_is_visible = Signal::new(true);

  // Initial load of player window
  use_effect(move || {
    if open_windows
      .iter()
      .find(|item| item.id == "based-radio")
      .is_none()
    {
      open_windows.push(OpenWindow {
      id: "based-radio".to_string(),
      window: rsx! { Player { is_visible: player_is_visible } },
      taskbar_item: rsx! { TaskbarItem { id: "based-radio", icon: None, title: "BasedRadio", is_visible: player_is_visible }},
    })
    }
  });

  // load news
  use_effect(move || news_loader(news_is_visible));

  let drag_state = radio_state.drag_state;
  let mut is_dragging = drag_state.is_dragging;

  let background_img = player_state.background;

  let mut dim_x = drag_state.dim_x;
  let mut dim_y = drag_state.dim_y;

  let mut previous_x = drag_state.previous_x;
  let mut previous_y = drag_state.previous_y;

  let mouse_move = move |event: Event<MouseData>| async move {
    if event.held_buttons().contains(MouseButton::Primary) && is_dragging() {
      // current mouse pos
      let screen_coords = event.screen_coordinates();
      // set previous to current if new
      if previous_x() == 0.0 {
        previous_x.set(screen_coords.x)
      }
      if previous_y() == 0.0 {
        previous_y.set(screen_coords.y)
      }

      let offset_x = previous_x() - screen_coords.x;
      let offset_y = previous_y() - screen_coords.y;

      let new_x = (dim_x() - offset_x).abs();
      let new_y = (dim_y() - offset_y).abs();

      dim_x.set(new_x);
      dim_y.set(new_y);

      // Finally, update the previous coords to the current pos
      previous_x.set(screen_coords.x);
      previous_y.set(screen_coords.y);
    }
  };

  rsx! {
    document::Title { "{player_state.title} | BasedRadio" },
    document::Script { src: asset!("/assets/wasm-check.js") }
    div {
      id: "main-container",
      class: "win98",
      style: "height: 100%; width: 100%; top: 0; left: 0; position: fixed;",
      style: if (settings_state.use_background)() && background_img().is_some() {"background-image: url({background_img().unwrap()});"},
      onmousemove: move |event| mouse_move(event),
      onmouseup: move |_| {
        is_dragging.set(false);
        if (settings_state.mouse_sounds)() {
          play_sound_effect("mouseup-audio");
        }
      },
      onmousedown: move |_| if (settings_state.mouse_sounds)() { play_sound_effect("mousedown-audio"); },
      audio { id: "mousedown-audio", src: mousedownsound },
      audio { id: "mouseup-audio", src: mouseupsound },
      Taskbar {},
      for item in open_windows.iter() {
        {item.window.clone()}
      }
    }
  }
}
