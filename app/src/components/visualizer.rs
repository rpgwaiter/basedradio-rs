use dioxus::prelude::*;

// TODO: maybe web only, probably not tho
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "web")]
use js_sys::Promise;
// #[cfg(feature = "web")]
use web_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure};
#[cfg(feature = "web")]
use web_sys::{
  AudioContext, AudioDestinationNode, CanvasRenderingContext2d, HtmlAudioElement,
  HtmlCanvasElement, HtmlElement, MediaElementAudioSourceNode, console, window,
};

// #[cfg(feature = "web")]
pub fn visualize() {
  let document = window().unwrap().document().unwrap();

  // This is gross, there has to be a better way
  let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
  let g = f.clone();

  let mut context = AudioContext::new().unwrap();
  let mut analyzer = context.create_analyser().unwrap();
  let mut gain_node = context.create_gain().unwrap();
  let canvas = document
    .get_element_by_id("player-visualizer")
    .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok())
    .unwrap();

  let mut canvas_context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap() // 2 unwraps???
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap();

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

    let num_points: i64 = (analyzer.fft_size() / 2) as i64;

    // let mut audio_data_array: Vec<u8> = Vec::with_capacity(num_points.try_into().unwrap());
    let mut audio_data_array: Vec<u8> = vec![0u8; num_points as usize];
    let generate = Closure::wrap(Box::new(move || {
      let clone = canvas.clone();
      let height = clone.height() as f64;
      let width = clone.width() as f64;
      canvas_context.clear_rect(0.0, 0.0, width.into(), height.into());
      analyzer.get_byte_frequency_data(&mut audio_data_array);

      // How big the bars are
      let SIZE = 15 as usize;

      // TODO: check state of visualizer toggle
      let width_int: i64 = width.round() as i64;
      if (audio_data_array.len() > 0) {
        for x in (0..width_int).step_by(SIZE as usize) {
          let ndx = ((x * num_points) / width_int) | 0;
          let audio_value = (audio_data_array[ndx as usize] / 255) as f64;
          let color = if audio_value > 0.45 {
            "#FF00AAAA"
          } else {
            "#FF00FFAA"
          };
          canvas_context.set_fill_style(&JsValue::from_str(color));
          let bar_height = (height - audio_value * height).ceil();
          if audio_value > 0.05 {
            canvas_context.fill_rect(x as f64, bar_height, SIZE as f64, height)
          }
        }
      }

      // generate next frame, continuously
      window()
        .unwrap()
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("error registering next animation frame in the visualizer for some reason");
    }) as Box<dyn FnMut()>);

    // store the generate function in g
    *g.borrow_mut() = Some(generate);

    window()
      .unwrap()
      .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
      .expect("error registering next animation frame in the visualizer for some reason");
  }
}

#[cfg(feature = "web")]
pub fn Visualizer() -> Element {
  rsx! {
    canvas {
      // I feel like i gotta be close but idk what im missing
      // onmounted: move |cx| {
      //   visualize()
      // },
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
