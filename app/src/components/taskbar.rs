use dioxus::{prelude::*, warnings::Warning};

use crate::components::{ICON_FAVICON, RadioState};

#[derive(PartialEq, Props, Clone)]
pub struct TaskbarItemProps {
  pub title: String,
  pub id: String,
  pub icon: Option<String>,
  pub is_visible: Signal<bool>,
}

#[component]
pub fn TaskbarItem(props: TaskbarItemProps) -> Element {
  let mut active_window = use_context::<RadioState>().drag_state.active_window;
  let is_active = active_window() == props.id;
  let mut is_visible = props.is_visible;

  rsx! {
    button {
      class: if is_active { "taskbar-item active-task" } else { "taskbar-item" },
      onclick: move |_| {
        warnings::copy_value_hoisted::allow(|| is_visible.toggle());
        active_window.set( if is_active { "based-radio".to_string() } else { props.id.clone() });
      }, // TODO: set last window
      div { class: "taskbar-icon icon", style: format!("background: url({}) no-repeat; ", props.icon.unwrap_or(ICON_FAVICON.to_string())) },
      "{props.title}"
    }
  }
}

#[component]
pub fn Taskbar() -> Element {
  let items = use_context::<RadioState>().open_windows;

  rsx! {
    div {
      class: "taskbar",
      for item in items.iter().map(|e| e.taskbar_item.clone()) {
        {item}
      }
    }
  }
}
