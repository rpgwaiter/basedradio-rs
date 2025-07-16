use dioxus::prelude::*;
use crate::components::{audio::RadioAudio, Visualizer, Window};
use crate::RadioState;


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
                  class: "inner content"
                }

            },

          }
        }
        
    }
}
