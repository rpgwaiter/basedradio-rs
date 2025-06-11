use dioxus::prelude::*;

pub fn Visualizer() -> Element {
    rsx! {
      canvas {
        class: "player-visualizer"
      }
    }
}
