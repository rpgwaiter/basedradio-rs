use crate::components::windows::WindowParentProps;
use crate::components::{OpenWindow, TaskbarItem, WindowTemplate};
use crate::{RadioState, SettingsState};

use dioxus::prelude::*;

#[component]
pub fn SettingsButton() -> Element {
  // let mut is_visible = use_context::<RadioState>().visibility.settings;
  let mut is_visible = Signal::new(true);
  let bounce = use_context::<SettingsState>().bounce;
  let mut active = use_context::<RadioState>().drag_state.active_window;
  let id = Signal::new("settings-window".to_string());
  let mut open_windows = use_context::<RadioState>().open_windows;

  rsx! {
    button {
      onclick: move |_| {
        if open_windows
          .iter()
          .find(|item| item.id == id() ).is_none() {
            open_windows.push(OpenWindow {
              id: id(),
              taskbar_item: rsx! {
                TaskbarItem {
                  id: id(),
                  title: "Settings".to_string(),
                  is_visible: is_visible,
                  icon: None,
                }
              },
              window: rsx! { SettingsWindow { is_visible: is_visible } }
            });
            active.set(id());
          } else {
            if !bounce() { is_visible.toggle() }
            active.set(if is_visible() { id() } else { "based-radio".to_string() } );
          };
      },
      id: "settings-btn",
      "Settings"
    }
  }
}

#[component]
pub fn SettingsWindow(props: WindowParentProps) -> Element {
  let mut settings_state = use_context::<SettingsState>();
  let bounce = use_context::<SettingsState>().bounce;

  rsx! {
    // if (props.is_visible() || bounce() {
      WindowTemplate {
        title: "Settings",
        id: "settings-window",
        header_icon: true,
        is_visible: props.is_visible,
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
    // }
  }
}
