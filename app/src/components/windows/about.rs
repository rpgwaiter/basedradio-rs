use crate::RadioState;
use crate::components::{Visualizer, WindowTemplate, TaskbarItemProps};
use dioxus::prelude::*;

// If this is the direction i end up going in, i should move this to a different file
#[derive(PartialEq, Props, Clone)]
struct GenericWindowProps {
  is_visible: Signal<bool>
}

#[component]
pub fn AboutButton() -> Element {
  let mut is_visible = use_signal(|| true);
  let mut active = use_context::<RadioState>().drag_state.active_window;

  let mut taskbar_items = use_context::<RadioState>().taskbar_items;

  rsx! {
    a {
      onclick: move |_| {
        if taskbar_items
          .iter()
          .find(|item| item.id == "window-about" ).is_none() {
            taskbar_items.push(TaskbarItemProps {
              id: "window-about".to_string(),
              title: "About".to_string(),
              is_visible: is_visible,
              icon: None,
              el: rsx! { AboutWindow { is_visible: is_visible } }
            });
          } else {
            is_visible.toggle();
          };
        active.set(if is_visible() { "window-about".to_string() } else { "based-radio".to_string() } );
      },
      role: "button",
      "About"
    }
  }
}

#[component]
pub fn AboutWindow(props: GenericWindowProps) -> Element {
  let mut taskbar_items = use_context::<RadioState>().taskbar_items;

  rsx! {
    WindowTemplate {
      title: "About",
      id: "window-about",
      header_icon: true,
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
