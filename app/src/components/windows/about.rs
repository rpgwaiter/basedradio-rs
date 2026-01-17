use crate::RadioState;
use crate::components::windows::WindowParentProps;
use crate::components::{ICON_FAVICON, OpenWindow, TaskbarItem, WindowTemplate};
use dioxus::prelude::*;

#[component]
pub fn AboutButton() -> Element {
  let mut active = use_context::<RadioState>().drag_state.active_window;
  let mut open_windows = use_context::<RadioState>().open_windows;
  let mut is_visible = Signal::new(true);

  let id = Signal::new("window-about".to_string());

  rsx! {
    button {
      class: "button-about",
      onclick: move |_| {
        if open_windows
          .iter()
          .find(|item| item.id == id() ).is_none() {
            open_windows.push(OpenWindow {
              id: id(),
              taskbar_item: rsx! {
                TaskbarItem {
                  id: id(),
                  title: "About".to_string(),
                  is_visible: is_visible
                }
              },
              window: rsx! { AboutWindow { is_visible: is_visible } }
            });
            active.set(id());
          } else {
            is_visible.toggle();
            active.set(if is_visible() { id() } else { "based-radio".to_string() } );
          };
      },
      // role: "button",
      // "?"
    }
  }
}

#[component]
pub fn AboutWindow(props: WindowParentProps) -> Element {
  rsx! {
    WindowTemplate {
      title: "About BasedRadio",
      id: "window-about",
      header_icon: ICON_FAVICON,
      is_visible: props.is_visible,
      index: 4,
      div {
        id: "about-radio",
        class: "inner content",
        h3 { "BasedRadio is an internet radio station playing classic and obscure music from the pre-32bit era." },
        br {},
        h4 { "All of the code for this site is custom (and written in rust using the Dioxus framework)." },
        br {},
        p { "If you're interested: ", a { href: "https://github.com/rpgwaiter/basedradio-rs", target: "_blank", rel: "noopener noreferrer", "source code" } }
        br {},
        h4 { "-- No AI/LLM tools were used in the making of BasedRadio. --" },
      }
    }
  }
}
