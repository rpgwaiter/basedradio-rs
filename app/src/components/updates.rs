use crate::RadioState;
use crate::components::{Visualizer, Window, audio::RadioAudio};
use dioxus::prelude::*;

#[component]
pub fn Updates() -> Element {
  let mut is_visible = use_context::<RadioState>().updates_is_visible;
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
              li { "[1 Oct 2025] Rewrote the api in rust, moved back to using icecast" }
              li { "[17 Jul 2025] This rewrite is a very early WIP. Stay tuned" }
            }
          }
        },
      }
    }
  }
}
