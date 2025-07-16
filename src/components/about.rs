use crate::RadioState;
use crate::components::{Visualizer, Window, audio::RadioAudio};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
  let mut isVisible = use_context::<RadioState>().aboutIsVisible;
  rsx! {
      if isVisible() {
        div {
          id: "window-player",
          class: "win98",
          style: "z-index: 4 !important;",
          Window {
              title: "About",
              id: "about-window",
              header_icon: true,
              isVisible: isVisible,
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
}
