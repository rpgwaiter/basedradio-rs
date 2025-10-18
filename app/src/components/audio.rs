use crate::components::get_stream_mp3;
use dioxus::prelude::*;

#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{HtmlAudioElement, window};

#[cfg(feature = "desktop")]
use rodio::{Decoder, OutputStream, Sink};
#[cfg(feature = "desktop")]
use std::thread;
#[cfg(feature = "desktop")]
use stream_download::http::HttpStream;
#[cfg(feature = "desktop")]
use stream_download::http::reqwest::Client;
#[cfg(feature = "desktop")]
use stream_download::source::{DecodeError, SourceStream};
#[cfg(feature = "desktop")]
use stream_download::storage::temp::TempStorageProvider;
#[cfg(feature = "desktop")]
use stream_download::{Settings, StreamDownload};

#[cfg(feature = "web")]
pub async fn play_audio() {
  let document = window().unwrap().document().unwrap();

  if let Some(audio) = document
    .get_element_by_id("main-audio")
    .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
  {
    if audio.paused() {
      audio.set_src(&get_stream_mp3());
      audio.load();
      audio.play();
    } else {
      audio.pause();
    }
  };
}

// TODO: update button text
#[cfg(feature = "desktop")]
pub async fn play_audio() {
  println!("attemting to play audio");
  let stream = HttpStream::<Client>::create(get_stream_mp3().parse().unwrap())
    .await
    .unwrap();

  println!("content length={:?}", stream.content_length());
  println!("content type={:?}", stream.content_type());

  let reader = match StreamDownload::from_stream(
    stream,
    TempStorageProvider::new(),
    Settings::default(),
  )
  .await
  {
    Ok(reader) => reader,
    Err(e) => return Err(e.decode_error().await).unwrap(),
  };

  thread::spawn(move || {
    if let Ok((_stream, handle)) = OutputStream::try_default() {
      if let Ok(sink) = Sink::try_new(&handle) {
        if let Ok(source) = Decoder::new(reader) {
          sink.append(source);
          sink.sleep_until_end();
        }
      }
    }
  });
}

// Get raw audio bytestream for use in the visualizer
// TODO:
#[cfg(feature = "web")]
pub async fn get_audio_stream() {
  let document = window().unwrap().document().unwrap();

  // If we have a valid audio obj
  //     if let Some(audio) = document
  //         .get_element_by_id("main-audio")
  //         .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
  //     {
  //         audio
  //     }
}

#[component]
pub fn RadioAudio() -> Element {
  let mut play_button_text = use_signal(|| "Play");
  let mut audio_num = use_signal(|| 0 as i8);
  let mut audio_url = use_signal(|| format_url(audio_num()));

  // url param cache buster int
  fn format_url(num: i8) -> String {
    return format!("{}?t={:?}", get_stream_mp3(), num);
  }

  rsx! {

    div {
      audio {
        id: "main-audio",
        onplay: move |_| play_button_text.set("Pause"),
        onpause: move |_| {
          audio_num += 1;
          audio_url.set(format_url(audio_num()));
          play_button_text.set("Play");
        },
        // onloadstart: move |_| play_button_text.set("Loading..."),
        src: "{audio_url}"
      },
      button {
        onclick: |_| play_audio(),
        id: "play-btn",
        "{play_button_text}"
      }
    }
  }
}
