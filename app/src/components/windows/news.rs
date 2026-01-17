// The idea is that new update stuff can be added to the api in a cerdtain format,
// And the window will display whatever it says

use crate::components::{
  ICON_WARNING, OpenWindow, RadioState, Taskbar, TaskbarItem, WindowTemplate,
  windows::WindowParentProps,
};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct NewsWindowProps {
  is_visible: Signal<bool>,
}

// eventually this will pull from an api but for now this is fine
pub fn news_loader(mut vis: Signal<bool>) {
  let mut already_spawned = Signal::new(false);
  let mut open_windows = use_context::<RadioState>().open_windows;

  use_effect(move || {
    if open_windows.iter().find(|item| item.id == "news").is_none() && !already_spawned() {
      already_spawned.set(true);
      open_windows.push(OpenWindow {
      id: "news".to_string(),
      window: rsx! { NewsWindow { is_visible: vis } },
      taskbar_item: rsx! { TaskbarItem { id: "news", icon: ICON_WARNING, title: "News", is_visible: vis }},
    })
    }
  });
}

#[component]
pub fn NewsWindow(props: NewsWindowProps) -> Element {
  rsx! {
    WindowTemplate {
      title: String::from("I need your help!"),
      id: String::from("news"),
      header_icon: ICON_WARNING,
      index: 100,
      is_visible: props.is_visible,
      // x_offset: -20,
      y_offset: -30,
      div {
        class: "inner content",
        div {
          style: "padding: 2px; font-size: 12px;",
          p {"Do you have issues with playback in your browser? Please let me know! Include your browser version and OS. I'm unable to replicate but playback seems broken on some setups."},
          br {},
          strong {
            "Report broken playback on our ",
            a { href: "https://github.com/rpgwaiter/basedradio-rs/issues", "github issue tracker" }
          },
          br {},
          br {},
          strong {"Or send me an email: ", a { href: "mailto:bugs@based.radio", "bugs@based.radio" } }

        }
      }
    }
  }
}
