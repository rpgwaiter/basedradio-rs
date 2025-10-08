use dioxus::prelude::*;

// #[cfg(feature = "web")]
use web_sys::{HtmlAudioElement, HtmlCanvasElement, HtmlElement, AudioContext, MediaElementAudioSourceNode, console, window, wasm_bindgen::JsCast};

// Will revisit this later when dioxus fixes feature management in their cli
// #[cfg(feature = "web")]
// #[allow(non_snake_case)]
// pub fn Visualizer() -> Element {
//   let document = window().unwrap().document().unwrap();

//   let mut context = AudioContext::new().unwrap();
//   let mut analyzer = context.create_analyser().unwrap();
//   let mut gain_node = context.create_gain().unwrap();
//   let canvas = document
//     .get_element_by_id("player-visualizer")
//     .unwrap()
//     .dyn_into::<HtmlCanvasElement>();
//     // .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok());
  

//   if let Some(audio) = document
//       .get_element_by_id("main-audio")

//       // .unwrap()
//       .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
//   {
//     // Audio exists
//     let source: MediaElementAudioSourceNode = context.create_media_element_source(&audio).unwrap();
//     source.connect_with_audio_node(&analyzer);
//     analyzer.connect_with_audio_node(&gain_node);

//   }

//   rsx! {
//     canvas {
//       id: "player-visualizer",
//       class: "player-visualizer",
//     }
//   }
// }

// #[cfg(feature = "desktop")]
#[allow(non_snake_case)]
pub fn Visualizer() -> Element {
  rsx! {
    canvas {
      id: "player-visualizer",
      class: "player-visualizer"
    }
  }
}