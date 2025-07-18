mod components;
use components::{About, Player, PlayerState, RadioState};
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
  let state = use_context_provider(|| RadioState::new());
  rsx! {
      About {},
      Player {}
  }
}
