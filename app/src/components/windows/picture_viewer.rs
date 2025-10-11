use dioxus::prelude::*;

use crate::RadioState;
use crate::components::{Visualizer, WindowTemplate};
use dioxus::prelude::*;

#[component]
pub fn PictureButton() -> Element {
  let mut is_visible = use_context::<RadioState>().about_is_visible;

  rsx! {
    a {
      onclick: move |event| is_visible.toggle(),
      role: "button",
    }
  }
}

#[component]
pub fn PictureWindow() -> Element {
  let is_visible = use_context::<RadioState>().about_is_visible;

  rsx! {
    if is_visible() {
      div {
        class: "win98",
        style: "z-index: 5 !important;",
        WindowTemplate {
          title: "Picture Viewer",
          id: "window-picture-viewer",
          header_icon: true,
          is_visible: is_visible,
          div {
            class: "inner content",
            h1 {"test"}
          }
        },
      }
    }
  }
}
