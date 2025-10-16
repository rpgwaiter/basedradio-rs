use crate::RadioState;
use crate::components::{MoreInfoState, UpstreamMoreInfo, WindowTemplate, get_api_url};
use dioxus::prelude::*;
use std::env;

#[component]
pub fn MoreInfoButton() -> Element {
  let mut is_visible = use_context::<RadioState>().visibility.more_info;
  let mut more = use_context::<MoreInfoState>();
  let mut active = use_context::<RadioState>().drag_state.active_window;

  let get_more_info = move || async move {
    if let Ok(response) = reqwest::get(format!("{}/more-info", get_api_url()))
      .await
      .unwrap()
      .json::<UpstreamMoreInfo>()
      .await
    {
      more.more_info.set(response);
    }
  };

  rsx! {
    button {
      onclick: move |_| {
        let v = is_visible();
        if (!v) { spawn(get_more_info()); };
        active.set(if !v { "window-more-info".to_string() } else { "based-radio".to_string() } );
        is_visible.toggle()
      },
      id: "more-info-btn",
      "More Info"
    }
  }
}

#[component]
pub fn MoreInfoWindow() -> Element {
  let more_info = use_context::<MoreInfoState>().more_info;

  rsx! {
    WindowTemplate {
      title: "More Info",
      id: "window-more-info",
      header_icon: true,
      is_visible: use_context::<RadioState>().visibility.more_info,
      index: 2,
      div {
        id: "more-info-radio",
        class: "inner content",
        // TODO: add info here
        div {
          h2 { style: "text-align: center;", u { "- Fun Fact -" }  },
          p { "{more_info().notes[0]}" }// TODO: randomize cross-platform
        }
      }
    }
  }
}
