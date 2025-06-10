use crate::components::{audio::RadioAudio, Window};
use dioxus::{html::u::rest, prelude::*};

use dioxus_sdk::utils::timing::use_interval;
use std::time::Duration;

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
    cover: String,
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
                id: "about-show",
                role: "button",
                "About"
            },
            div {
                class: "action",
                a {
                    id: "download-btn",
                    role: "button",
                    "Download"
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
    let mut cover_art = use_signal(|| "".to_string());

    let fetch_info = move || async move {
        if let Ok(response) = reqwest::get(API_URL)
            .await
            .unwrap()
            .json::<RadioApi>()
            .await
        {
            game.set(response.song.game);
            track.set(response.song.title);
            system.set(response.song.system);
            cover_art.set(response.song.cover);
            // There just has to be a better way
            elapsed.set(response.status.elapsed.parse::<f32>().unwrap().round() as i16);
            duration.set(response.status.duration.parse::<f32>().unwrap().round() as i16);
        }
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
        div {
            class: "stream-meta",
            div {
                class: "player-cover-art",
                img { id: "current-cover", src: "{cover_art}", alt: "Cover Art" }
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
            RadioAudio {  }
        }
    }
}

#[component]
pub fn Player() -> Element {
    rsx! {
        div {
            id: "window-player",
            class: "win98",
            Window {
                title: "BasedRadio",
                id: "based-radio",
                PlayerMenu {  },
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
