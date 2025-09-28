use crate::RadioState;
use crate::components::{Visualizer, Window, audio::RadioAudio};
use dioxus::prelude::*;

#[component]
pub fn Updates() -> Element {
  let mut isVisible = use_context::<RadioState>().updatesIsVisible;
  rsx! {
    if isVisible() {
      div {
        id: "window-updates",
        class: "win98",
        style: "z-index: 2 !important;",
        Window {
          title: "Updates",
          id: "update-window",
          header_icon: true,
          isVisible: isVisible,
          div {
            id: "updates",
            class: "inner content",
            ul {
              li { "[Jul 17 2025] This rewrite is a very early WIP. Stay tuned" }
            }
          }
        },
      }
    }
  }
}
