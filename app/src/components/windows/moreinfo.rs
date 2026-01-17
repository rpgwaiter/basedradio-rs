use crate::RadioState;
use crate::components::windows::WindowParentProps;
use crate::components::{
  ICON_FAVICON, MoreInfoState, OpenWindow, TaskbarItem, UpstreamMoreInfo, WindowTemplate,
  get_api_url,
};
use dioxus::prelude::*;

#[component]
pub fn MoreInfoButton() -> Element {
  let mut is_visible = Signal::new(true);
  let mut more = use_context::<MoreInfoState>();
  let mut active = use_context::<RadioState>().drag_state.active_window;
  let mut open_windows = use_context::<RadioState>().open_windows;

  let id = Signal::new("window-more-info".to_string());

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
        if open_windows
          .iter()
          .find(|item| item.id == id() ).is_none() {
            open_windows.push(OpenWindow {
              id: id(),
              taskbar_item: rsx! {
                TaskbarItem {
                  id: id(),
                  title: "More Info".to_string(),
                  is_visible: is_visible
                }
              },
              window: rsx! { MoreInfoWindow { is_visible: is_visible } }
            });
            active.set(id());
          } else {
            is_visible.toggle();
            active.set(if is_visible() { id() } else { "based-radio".to_string() } );
          };
        spawn(get_more_info());
      },
      id: "more-info-btn",
      "More Info"
    }
  }
}

#[component]
pub fn MoreInfoWindow(props: WindowParentProps) -> Element {
  let more_info = use_context::<MoreInfoState>().more_info;

  rsx! {
    WindowTemplate {
      title: "More Info",
      id: "window-more-info",
      header_icon: ICON_FAVICON,
      // is_visible: use_context::<RadioState>().visibility.more_info,
      is_visible: props.is_visible,
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
