use crate::RadioState;
use crate::components::{Visualizer, WindowTemplate};
use dioxus::prelude::*;

#[component]
pub fn AboutButton() -> Element {
  let mut is_visible = use_context::<RadioState>().about_is_visible;
  let mut active = use_context::<RadioState>().drag_state.active_window;

  rsx! {
    a {
      onclick: move |_| {
        active.set(if !is_visible() { "window-about".to_string() } else { "based-radio".to_string() } );
        is_visible.toggle()
      },
      role: "button",
      "About"
    }
  }
}

#[component]
pub fn AboutWindow() -> Element {
  let is_visible = use_context::<RadioState>().about_is_visible;
  rsx! {
    if is_visible() {
      WindowTemplate {
        title: "About",
        id: "window-about",
        header_icon: true,
        is_visible: is_visible,
        index: 4,
        div {
          id: "about-radio",
          class: "inner content",
          h3 {
            "BasedRadio is an internet radio station playing classic and obscure music from the pre-32bit era. Heavily inspired by ",
            a { href: "https://plaza.one", target: "_blank", rel: "noopener noreferrer", "plaza.one" },
            ", all of the code for this site is custom (and written in rust)."
          },
          br {},
          p { "If you're interested: ", a { href: "https://github.com/rpgwaiter/basedradio-rs", target: "_blank", rel: "noopener noreferrer", "source code" } }
          br {},
          h4 { "-- No AI/LLM tools were used in the making of BasedRadio. --" },
        }
      },
    }
  }
}
