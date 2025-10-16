// TODO: make this a more genreal image viewer
// not sure we'll ever need that, but it would be cool
use dioxus::prelude::*;

use crate::PlayerState;
use crate::components::{RadioState, Visualizer, WindowTemplate};
use dioxus::prelude::*;

#[component]
pub fn PictureButton() -> Element {
  let mut is_visible = use_context::<RadioState>().visibility.picture;
  let cover = use_context::<PlayerState>().cover;
  let mut active = use_context::<RadioState>().drag_state.active_window;

  rsx! {
    img {
      id: "current-cover",
      src: "{cover}",
      alt: "Cover Art",
      style: "margin: auto; display: block;",
      onclick: move |_| {
        active.set(if !is_visible() { "window-imgview".to_string() } else { "based-radio".to_string() } );
        is_visible.toggle()
      }
    }
  }
}

#[component]
pub fn PictureWindow() -> Element {
  let cover = use_context::<PlayerState>().cover;

  rsx! {
    WindowTemplate {
      title: "ImgView",
      id: "window-imgview",
      header_icon: false,
      is_visible: use_context::<RadioState>().visibility.picture,
      index: 6,
      extra_style: "max-height: 50% !important; max-width: 50% !important;",
      div {
        div {
          class: "content",
          img {
            id: "current-cover",
            src: "{cover}",
            alt: "Cover Art",
            style: "margin: auto; height: 100%; width: 100%; object-fit: cover;",
          }
        }
      }
    }
  }
}
