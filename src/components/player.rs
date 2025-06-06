use dioxus::{
    html::{track, u::rest},
    prelude::*,
};
use dioxus_sdk::utils::timing::use_interval;
use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use web_sys::{window, HtmlAudioElement};
use web_sys::wasm_bindgen::JsCast;

pub static STREAM_MP3: &str = "https://cast.based.radio/vgm.mp3";
pub static API_URL: &str = "https://api.based.radio";

#[derive(serde::Deserialize)]
struct Song {
    album: String,
    artist: String,
    file: String,
    duration: String, // Eventually will be a number
    game: String,
    system: String,
    title: String,
}

#[derive(serde::Deserialize)]
struct Status {
    elapsed: String,
    duration: String,
}

#[derive(serde::Deserialize)]
struct RadioApi {
    song: Song,
    status: Status,
}

// TODO: move to a lib
fn add_zeros(e: i16, t: usize) -> String {
    let s = e.to_string();
    format!("{:0>width$}", s, width = t)
}

fn format_time(e: i16) -> String {
    let e = e % 3600; // seconds within the hour
    let min = add_zeros(e / 60, 2);
    let sec = add_zeros(e % 60, 2);
    format!("{}:{}", min, sec)
}

fn play_audio() {
    let document = window().unwrap().document().unwrap();
    if let Some(audio) = document
        .get_element_by_id("main-audio")
        .and_then(|el| el.dyn_into::<HtmlAudioElement>().ok())
    {
        let _ = audio.play(); // Can handle result if you want
    }
}

// #[layout]
// pub fn Window(id: String) -> Element {
//     rsx! {
//         div {
//             id: "player",
//             div {
//                 id: "window-player",
//                 class: "win98",
//                 div {
//                     id: id,
//                     class: "window",
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn PlayerMenu() -> Element {
    rsx! {
        div {
            id: "player-menu",
            class: "menu-bar",
            div {
                class: "action",
                a {
                    id: "home-button",
                    href: "/",
                    role: "button",
                    u {"H"}, "ome"
                },
            },
            div {
                class: "action",
                a {
                    id: "about-show",
                    role: "button",
                    u {"A"}, "bout"
                }
            },
            div {
                class: "action",
                a {
                    id: "download-btn",
                    role: "button",
                    u {"D"}, "ownload"
                }
            },
            // div {
            //     class: "action",
            //     a {
            //         id: "updates-show",
            //         role: "button",
            //         style: "float: right;",
            //         u {"U"}, "pdates"
            //     }
            // }
        }
    }
}

#[component]
pub fn PlayerStats(system: Signal<String>, track: Signal<String>, game: Signal<String>) -> Element {
    // TODO: make this global to this player file
    // let mut game = use_signal(|| "Loading stream info...".to_string());
    // let mut track = use_signal(|| "".to_string());
    // let mut system = use_signal(|| "".to_string());

    // counter increment
    // use_effect(move || loop {
    //     elapsed.set(elapsed + 1 as i16);
    //     println!("Looping now");

    //     sleep(Duration::from_secs(1));
    // });

    // use_interva

    // fetch_info().await();

    rsx! {
        div {
            class: "player-stats",
            div {
                class: "player-game",
                strong { "Game: " }, a { id: "current-game", "{game}" }
            },
            div {
                class: "player-track",
                strong { "Track: " }, a { id: "current-track", "{track}" }
            },
            div {
                class: "player-system",
                strong { "System: " }, a { id: "current-system", "{system}" }
            }
        },
        div {

        }
    }
}

// #[component]
// pub fn Visualizer() -> Element {
//     rsx! {
//         script {

//         }
//     }
// }

#[component]
pub fn PlayerContent() -> Element {
    let mut elapsed = use_signal(|| 0 as i16);
    let mut duration = use_signal(|| 0 as i16);
    let mut game = use_signal(|| "Loading stream info...".to_string());
    let mut track = use_signal(|| "".to_string());
    let mut system = use_signal(|| "".to_string());

    let fetch_info = move || async move {
        let response = reqwest::get(API_URL)
            .await
            .unwrap()
            .json::<RadioApi>()
            .await
            .unwrap();

        game.set(response.song.game);
        track.set(response.song.title);
        system.set(response.song.system);
        // There just has to be a better way
        elapsed.set(response.status.elapsed.parse::<f32>().unwrap().round() as i16);
        duration.set(response.status.duration.parse::<f32>().unwrap().round() as i16);
    };

    // Initial load
    // This if ensures that we don't spam the api
    // TODO: track this better. If the api is dead it will get spammed
    if track.peek().as_str() == "" {
        print!("laoding thing");
        spawn(fetch_info());
    };

    use_interval(Duration::from_secs(1), move || {
        if elapsed() >= duration() {
            spawn(fetch_info());
        };
        elapsed += 1
    });

    rsx! {
        audio {
            id: "main-audio",
            src: STREAM_MP3
        },
        div {
            class: "stream-meta",
            div {
                class: "player-cover-art",
                img { id: "current-cover", alt: "Cover Art" }
            },
            PlayerStats { game: game, system: system, track: track  }
        },
        div {
            class: "player-meta",
            div {
                class: "player-time-container text-field",
                div {
                    id: "player-time",
                    "~~~ ",
                    a { id: "elapsed-time", "{format_time(*elapsed.read())}" } " / " a { id: "song-duration", "{format_time(*duration.read())}"}
                    " ~~~"
                }
            },
            div {
              class: "content-buttons",
              button { onclick: move |_| play_audio(), id: "play-btn", u {"P"}, "lay" }
            }
        }
    }
}

// TODO: make this a layout
#[component]
pub fn Player() -> Element {
    rsx! {
        div {
            id: "player",
            div {
                id: "window-player",
                class: "win98",
                div {
                    id: "based-radio",
                    class: "window",
                    div {
                        class: "header",
                        div { class: "icon" },
                        "BasedRadio",
                        div {
                            class: "buttons",
                            button { class: "button-minimize" }
                        }
                    },
                    PlayerMenu {  }
                    div {
                        class: "inner",
                        div {
                            id: "player-container",
                            class: "minimizable content",
                            PlayerContent {  }
                        }
                    },
                    div {
                        class: "player-footer",
                        div { "Keep it Based." },
                        div { class: "footer-end" }
                    }
                }
            }
        }
    }
}
