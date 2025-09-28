use crate::RadioState;
use crate::components::{Visualizer, Window, audio::RadioAudio, API_URL};
use dioxus::prelude::*;


// pub struct RegionalTitles {
//   en: Option<String>,
//   jp: Option<String>,
//   others: Option<<Vec<String>>,
// }

// // External links related to games
// pub struct InfoLinks {
//   wikipedia: Option<String>,
//   khinsider: Option<String>,
// }

#[derive(Clone, serde::Deserialize)]
pub struct TrackMoreInfoUpstream {
  notes: Vec<String>
  // game: RegionalTitles,
  // links: InfoLinks
  // notes: Vec<String> // Site owner's notes. Fun facts, random stuff
}

#[derive(Clone)]
pub struct TrackMoreInfo {
  notes: Signal<Vec<String>>
  // game: RegionalTitles,
  // links: InfoLinks
  // notes: Vec<String> // Site owner's notes. Fun facts, random stuff
}



impl TrackMoreInfo {
  pub fn new() -> Self {
    TrackMoreInfo {
      notes: Signal::new(vec![String::from("Default note, you should never see this")])
    }
  }
}

#[component]
pub fn MoreInfoButton() -> Element {
  let mut isVisible = use_context::<RadioState>().moreInfoIsVisible;
  let mut notes = use_context::<TrackMoreInfo>().notes;

  let get_more_info = move || async move {
    if let Ok(response) = reqwest::get(format!("{}/more-info", API_URL))
      .await
      .unwrap() // TODO: handle a dead api
      .json::<TrackMoreInfoUpstream>()
      .await
      {
        let n = response.notes[0].clone();
        println!("NOTE REEE: {:?}", n);
        notes.set(response.notes);
        // println!("NOTE REEE3: {:?}", n);
      }
  };

  rsx! {
    button {
      onclick: move |event| {
        spawn(get_more_info());
        isVisible.toggle()
      },
      id: "more-info-btn",
      "More Info"
    }
  }
}

#[component]
pub fn MoreInfo() -> Element {
  println!("Rendering more info");
  let mut isVisible = use_context::<RadioState>().moreInfoIsVisible;
  let mut notes = use_context::<TrackMoreInfo>().notes;
  let mut initialLoad = Signal::new(false);


  println!("notes is: {:?}", notes());

  rsx! {
    if isVisible() {
      div {
        // id: "window-more-info",
        class: "win98",
        style: "z-index: 2 !important;",
        Window {
          title: "More Info",
          id: "more-info-window",
          header_icon: true,
          isVisible: isVisible,
          div {
            id: "more-info-radio",
            class: "inner content",
            // TODO: add info here
            div {
              h2 { style: "text-align: center;", u { "- Fun Fact -" }  },
              p { "{notes()[0]}" }// TODO: randomize cross-platform

            }
          }
        },
      }
    }
  }
}
