mod components;
use components::windows::{AboutWindow, MoreInfoWindow, Player, SettingsWindow, UpdatesWindow};
use components::{MoreInfoState, PlayerState, RadioState, SettingsState, UpstreamMoreInfo};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// TODO: unify and clean these up
const BASED_RADIO_CSS: Asset = asset!("/assets/style/based98.css");
const APP_CSS: Asset = asset!("/assets/style/app.css");

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: APP_CSS }
      document::Link { rel: "stylesheet", href: BASED_RADIO_CSS }
      Router::<Route> {}
  }
}

/// Home page
#[component]
fn Home() -> Element {
  use_context_provider(|| RadioState::new());
  use_context_provider(|| MoreInfoState::new());
  use_context_provider(|| SettingsState::new());
  let player_state = use_context_provider(|| PlayerState::new());

  let bg_toggle = use_context::<SettingsState>().use_background;
  let background_img = player_state.background;

  rsx! {
    div {
      id: "main-container",
      style: if (bg_toggle() && background_img().is_some()) {"background-image: url({background_img().unwrap()});"},
      AboutWindow {},
      Player {},
      UpdatesWindow {},
      MoreInfoWindow {},
      SettingsWindow {}
    }
  }
}
