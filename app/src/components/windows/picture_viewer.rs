// TODO: make this a more genreal image viewer
// not sure we'll ever need that, but it would be cool
use dioxus::prelude::*;

use crate::PlayerState;
use crate::components::{RadioState, Visualizer, WindowTemplate};
use dioxus::prelude::*;

#[component]
pub fn PictureButton() -> Element {
  let mut radio_state = use_context::<RadioState>();
  let cover = use_context::<PlayerState>().cover;

  rsx! {
    img {
      id: "current-cover",
      src: "{cover}",
      alt: "Cover Art",
      style: "margin: auto; display: block;",
      onclick: move |_| {
        if !(radio_state.picture_is_visible)() {}
        (radio_state.picture_is_visible).toggle()
      }
    }
  }
}

#[component]
pub fn PictureWindow() -> Element {
  let is_visible = Signal::new(false);

  rsx! {
    if is_visible() {
      WindowTemplate {
        title: "Picture Viewer",
        id: "window-picture-viewer",
        header_icon: true,
        is_visible: is_visible,
        index: 6,
        div {
          class: "inner content",
          h1 {"test"}
        }
      }
    }
  }
}
