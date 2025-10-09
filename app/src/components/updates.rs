use crate::RadioState;
use crate::components::{Visualizer, Window, audio::RadioAudio, get_api_url};
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct Updates {
  updates: Vec<String>,
}

#[component]
pub fn UpdatesButton() -> Element {
  let mut updates_is_visible = use_context::<RadioState>().updates_is_visible;
  let mut updates = use_context::<RadioState>().updates;

  let fetch_info = move || async move {
    if let Ok(response) = reqwest::get(format!("{}/updates", get_api_url()))
      .await
      .unwrap()
      .json::<Updates>()
      .await
    {
      updates.set(response.updates)
    }
  };

  rsx! {
    a {
      onclick: move |_| {
        updates_is_visible.toggle();
        spawn(fetch_info());
      },
      id: "updates-show",
      role: "button",
      "Updates"
    }
  }
}

#[component]
pub fn UpdatesWindow() -> Element {
  let is_visible = use_context::<RadioState>().updates_is_visible;
  let updates = use_context::<RadioState>().updates;

  rsx! {
    if is_visible() {
      div {
        id: "container-updates",
        class: "win98",
        style: "z-index: 2 !important;",
        Window {
          title: "Updates",
          id: "window-updates",
          header_icon: true,
          is_visible: is_visible,
          div {
            id: "updates",
            class: "inner content",
            ul {
              for update in updates().iter() {
                li { "{update}" }
              }
            }
          }
        },
      }
    }
  }
}
