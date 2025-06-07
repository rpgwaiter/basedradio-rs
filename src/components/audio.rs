use dioxus::prelude::*;

#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;

#[cfg(feature = "web")]
use web_sys::{window, HtmlAudioElement, HtmlElement};

#[cfg(feature = "desktop")]
use rodio::{Decoder, OutputStream, Sink};
#[cfg(feature = "desktop")]
use std::thread;
#[cfg(feature = "desktop")]
use stream_download::http::reqwest::Client;
#[cfg(feature = "desktop")]
use stream_download::http::HttpStream;
#[cfg(feature = "desktop")]
use stream_download::source::{DecodeError, SourceStream};
#[cfg(feature = "desktop")]
use stream_download::storage::temp::TempStorageProvider;
#[cfg(feature = "desktop")]
use stream_download::{Settings, StreamDownload};

pub static STREAM_MP3: &str = "https://cast.based.radio/vgm.mp3";

#[cfg(feature = "web")]
pub async fn play_audio() { // TODO: get element from event
    let document = window().unwrap().document().unwrap();

    if let Some(audio) = document
        .get_element_by_id("main-audio")
        .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
    {
        
        audio.load();
        let _ = audio.play(); // Can handle result if you want

        if let Some(buttonEl) = document.get_element_by_id("play-btn").and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
          println!("Got the button")
        }
        
    }
}


#[cfg(feature = "desktop")]
pub async fn play_audio() {
    println!("attemting to play audio");
    let stream = HttpStream::<Client>::create(STREAM_MP3.parse().unwrap())
        .await
        .unwrap();

    println!("content length={:?}", stream.content_length());
    println!("content type={:?}", stream.content_type());

    let reader =
        match StreamDownload::from_stream(stream, TempStorageProvider::new(), Settings::default())
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

#[component]
pub fn RadioAudio() -> Element {
    rsx! {
        audio {
            id: "main-audio",
            src: STREAM_MP3
        },
        div {
          class: "content-buttons",
          button {
              onclick: |event| play_audio(),
              id: "play-btn",
              u {"P"}, "lay"
          }
        }
    }
}
