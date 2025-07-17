use dioxus::prelude::*;

pub fn Visualizer() -> Element {
  rsx! {
    canvas {
      id: "player-visualizer",
      class: "player-visualizer"
    }
  }
}
