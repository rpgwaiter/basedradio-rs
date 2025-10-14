use dioxus::prelude::*;

use crate::components::{DragState, ICON_FAVICON, RadioState};

#[derive(PartialEq, Props, Clone)]
pub struct TaskbarItemProps {
  pub title: String,
  pub id: String,
  pub icon: Option<String>,
}

#[component]
pub fn TaskbarItem(props: TaskbarItemProps) -> Element {
  let mut active_window = use_context::<RadioState>().drag_state.active_window;
  let is_active = active_window() == props.id;

  rsx! {
    button {
      class: if is_active { "taskbar-item active-task" } else { "taskbar-item" },
      onclick: move |_| active_window.set( if is_active { "based-radio".to_string() } else { props.id.clone() }), // TODO: set last window
      // TODO: move most of this style to css
      div { class: "icon", style: format!("background: url({}) no-repeat; background-size: cover; height: 18px; width: 18px; margin-left: 2px !important; margin-right: 2px !important;", props.icon.unwrap_or(ICON_FAVICON.to_string())) },
      "{props.title}"
    }
  }
}

#[component]
pub fn Taskbar() -> Element {
  let mut items = use_context::<RadioState>().taskbar_items;

  rsx! {
    div {
      class: "taskbar",
      for item in items.iter() {
        TaskbarItem { title: item.title.clone(), id: item.id.clone() }
      }
    }
  }
}
