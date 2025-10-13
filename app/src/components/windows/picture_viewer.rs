use dioxus::prelude::*;

use crate::RadioState;
use crate::components::{Visualizer, WindowTemplate};
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct PictureProps {
  pub image: String,
  pub id: Uuid,
  // is_visible: Option<Signal<bool>>
}

impl PictureProps {
  pub fn new(image: String) -> Self {
    PictureProps {
      image: image,
      id: Uuid::new_v4(),
      // is_visible: Signal::new(false)
    }
  }
}

#[component]
pub fn PictureButton(props: PictureProps) -> Element {
  let mut is_visible = Signal::new(false);

  let i = props.image;

  // let window_props = PictureProps::new(props.image);

  rsx! {
    img {
      id: "current-cover",
      src: "{i.clone()}",
      alt: "Cover Art",
      style: "margin: auto; display: block;",
      onclick: move |_| is_visible.toggle()
    },
    // if is_visible() {
    //   PictureWindow { image: i.clone() }
    // }
  }
}

#[component]
pub fn PictureWindow(props: PictureProps) -> Element {
  let is_visible = Signal::new(false);

  rsx! {
    // if is_visible() {
    div {
      class: "win98",
      style: "z-index: 5 !important;",
      WindowTemplate {
        title: "Picture Viewer",
        id: "window-picture-viewer",
        header_icon: true,
        is_visible: is_visible,
        index: 5,
        div {
          class: "inner content",
          h1 {"test"}
        }
      },
    }
  }
}
