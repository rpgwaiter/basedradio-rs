use dioxus::prelude::*;

#[cfg(feature = "web")]
use js_sys::Promise;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{
  AudioContext, AudioDestinationNode, HtmlAudioElement, HtmlCanvasElement, HtmlElement,
  MediaElementAudioSourceNode, console, window,
};

#[cfg(feature = "web")]
pub async fn visualize() {
  let document = window().unwrap().document().unwrap();
  println!("we out here visualizing");

  let mut context = AudioContext::new().unwrap();
  let mut analyzer = context.create_analyser().unwrap();
  let mut gain_node = context.create_gain().unwrap();
  let canvas = document
    .get_element_by_id("player-visualizer")
    // .unwrap()
    // .dyn_into::<HtmlCanvasElement>();
    .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok());

  let canvasContext = canvas.context("2d");

  if let Some(audio) = document
    .get_element_by_id("main-audio")
    // .unwrap()
    .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
  {
    // Audio exists
    let source: MediaElementAudioSourceNode = context.create_media_element_source(&audio).unwrap();
    source.connect_with_audio_node(&analyzer);
    analyzer.connect_with_audio_node(&gain_node);
    gain_node.connect_with_audio_node(&context.destination());

    let num_points = analyzer.fft_size() / 2;

    fn generate() {}
  }
}

#[allow(non_snake_case)]
#[cfg(feature = "web")]
pub fn Visualizer() -> Element {
  // spawn(visualize());

  rsx! {
    canvas {
      id: "player-visualizer",
      class: "player-visualizer"
    }
  }
}

#[allow(non_snake_case)]
#[cfg(feature = "desktop")]
pub fn Visualizer() -> Element {
  rsx! {
    canvas {
      id: "player-visualizer",
      class: "player-visualizer"
    }
  }
}
