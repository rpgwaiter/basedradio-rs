// TODO: make this a more genreal image viewer
// not sure we'll ever need that, but it would be cool
use dioxus::prelude::*;

use crate::PlayerState;
use crate::components::{RadioState, WindowTemplate, TaskbarItem, OpenWindow, windows::WindowParentProps};

#[component]
pub fn PictureButton() -> Element {
  let mut is_visible = Signal::new(true);
  let cover = use_context::<PlayerState>().cover;
  let mut active = use_context::<RadioState>().drag_state.active_window;
  let mut open_windows = use_context::<RadioState>().open_windows;

  let id = Signal::new("window-imgview".to_string());

  rsx! {
    img {
      id: "current-cover",
      src: "{cover}",
      alt: "Cover Art",
      style: "margin: auto; display: block;",
      onclick: move |_| {
        if open_windows
          .iter()
          .find(|item| item.id == id() ).is_none() {
            open_windows.push(OpenWindow {
              id: id(),
              taskbar_item: rsx! {
                TaskbarItem {
                  id: id(),
                  title: "ImgView".to_string(),
                  is_visible: is_visible,
                  icon: None,
                }
              },
              window: rsx! { PictureWindow { is_visible: is_visible } }
            });
            active.set(id());
          } else {
            is_visible.toggle();
            active.set(if is_visible() { id() } else { "based-radio".to_string() } );
          };
      },
    }
  }
}

#[component]
pub fn PictureWindow(props: WindowParentProps) -> Element {
  let cover = use_context::<PlayerState>().cover;

  rsx! {
    WindowTemplate {
      title: "ImgView",
      id: "window-imgview",
      header_icon: false,
      is_visible: props.is_visible,
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
