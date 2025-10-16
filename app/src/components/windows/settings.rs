use crate::components::WindowTemplate;
use crate::{RadioState, SettingsState};

use dioxus::prelude::*;
use std::env;

#[component]
pub fn SettingsButton() -> Element {
  let mut is_visible = use_context::<RadioState>().visibility.settings;
  let bounce = use_context::<SettingsState>().bounce;
  let mut active = use_context::<RadioState>().drag_state.active_window;

  rsx! {
    button {
      onclick: move |_| {
        active.set(if !is_visible() { "settings-window".to_string() } else { "based-radio".to_string() } );
        if !bounce() { is_visible.toggle() }
      },
      id: "settings-btn",
      "Settings"
    }
  }
}

#[component]
pub fn SettingsWindow() -> Element {
  let mut is_visible = use_context::<RadioState>().visibility.settings;
  let mut settings_state = use_context::<SettingsState>();
  let bounce = use_context::<SettingsState>().bounce;

  rsx! {
    if is_visible() || bounce() {
      WindowTemplate {
        title: "Settings",
        id: "settings-window",
        header_icon: true,
        is_visible: is_visible,
        index: 10,
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
