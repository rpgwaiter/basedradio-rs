use crate::components::windows::WindowParentProps;
use crate::RadioState;
use crate::components::{WindowTemplate, TaskbarItem, OpenWindow, get_api_url};
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct Updates {
  updates: Vec<String>,
}

#[component]
pub fn UpdatesButton() -> Element {
  let mut radio_state = use_context::<RadioState>();
  let mut active = radio_state.drag_state.active_window;
  let mut open_windows = radio_state.open_windows;

  let mut is_visible = Signal::new(true);
  let id = Signal::new("window-updates".to_string());

  let fetch_info = move || async move {
    if let Ok(response) = reqwest::get(format!("{}/updates", get_api_url()))
      .await
      .unwrap()
      .json::<Updates>()
      .await
    {
      radio_state.updates.set(response.updates)
    }
  };

  rsx! {
    a {
      onclick: move |_| {
        if open_windows
          .iter()
          .find(|item| item.id == id() ).is_none() {
            open_windows.push(OpenWindow {
              id: id(),
              taskbar_item: rsx! {
                TaskbarItem {
                  id: id(),
                  title: "Updates".to_string(),
                  is_visible: is_visible,
                  icon: None,
                }
              },
              window: rsx! { UpdatesWindow { is_visible: is_visible } }
            });
            active.set(id());
          } else {
            is_visible.toggle();
            active.set(if is_visible() { id() } else { "based-radio".to_string() } );
          };
          spawn(fetch_info());
      },
      id: "updates-show",
      role: "button",
      "Updates"
    }
  }
}

#[component]
pub fn UpdatesWindow(props: WindowParentProps) -> Element {
  let updates = use_context::<RadioState>().updates;

  rsx! {
    WindowTemplate {
      title: "Updates",
      id: "window-updates",
      header_icon: true,
      is_visible: props.is_visible,
      index: 2,
      div {
        id: "updates",
        class: "inner content",
        ul {
          for update in updates().iter() {
            li { "{update}" }
          }
        }
      }
    }
    
  }
}
