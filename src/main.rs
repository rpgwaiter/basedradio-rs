mod components;
use components::Player;
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
        document::Script { src: asset!("/assets/js/draggable.js") }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Player {}
    }
}
