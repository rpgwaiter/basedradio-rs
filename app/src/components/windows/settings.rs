use crate::components::WindowTemplate;
use crate::{RadioState, SettingsState};

use dioxus::prelude::*;
use std::env;

#[component]
pub fn SettingsButton() -> Element {
  let mut is_visible = use_context::<RadioState>().settings_is_visible;
  let bounce = use_context::<SettingsState>().bounce;

  rsx! {
    button {
      onclick: move |_| if !bounce() { is_visible.toggle() },
      id: "settings-btn",
      "Settings"
    }
  }
}

#[component]
pub fn SettingsWindow() -> Element {
  let is_visible = use_context::<RadioState>().settings_is_visible;
  let mut settings_state = use_context::<SettingsState>();
  let bounce = use_context::<SettingsState>().bounce;

  rsx! {
    if is_visible() || bounce() {
      WindowTemplate {
        title: "Settings",
        id: "settings-window",
        header_icon: true,
        is_visible: is_visible,
        index: 3,
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
              },
            }
            fieldset {
              input {
                id: "bounce-toggle",
                type: "checkbox",
                checked: settings_state.bounce,
                onclick: move |_| settings_state.bounce.toggle()
              },
              label {
                for: "bounce-toggle",
                {"Bounce"}
              },
            }
          }
        }
      },
    }
  }
}
