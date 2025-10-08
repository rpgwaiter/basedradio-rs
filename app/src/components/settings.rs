use crate::{RadioState, SettingsState};
use crate::components::{Window};

use dioxus::prelude::*;
use std::env;

#[component]
pub fn SettingsButton() -> Element {
  let mut is_visible = use_context::<RadioState>().settings_is_visible;
  // let mut more = use_context::<MoreInfoState>();

  rsx! {
    button {
      onclick: move |_| is_visible.toggle(),
      id: "settings-btn",
      "Settings"
    }
  }
}

#[component]
pub fn SettingsWindow() -> Element {
  let is_visible = use_context::<RadioState>().settings_is_visible;
  let mut settings_state = use_context::<SettingsState>();

  rsx! {
    if is_visible() {
      div {
        id: "settings-container",
        class: "win98",
        style: "z-index: 3 !important;",
        Window {
          title: "Settings",
          id: "settings-window",
          header_icon: true,
          is_visible: is_visible,
          div {
            class: "inner content",
            div {
              fieldset {
                input {
                  id: "background-toggle",
                  type: "checkbox",
                  checked: settings_state.use_background,
                  onclick: move |_| settings_state.use_background.toggle()
                },
                label {
                  for: "background-toggle",
                  {"Background"}
                }
              }
            }
          }
        },
      }
    }
  }
}
