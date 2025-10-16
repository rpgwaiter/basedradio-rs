use crate::components::{ICON_FAVICON, RadioAudio, RadioState, TaskbarItemProps, taskbar};
use core::task;
use dioxus::html::input_data::MouseButton;
use dioxus::logger::tracing::info;
use dioxus::{
  html::geometry::euclid::{Point2D, Rect, Vector2D},
  prelude::*,
};
use std::rc::Rc;

static ICON_CLOSE: Asset = asset!("/assets/ui/element2.png");
static RESIZE_ICON: Asset = asset!("/assets/ui/resize.png");

#[derive(PartialEq, Props, Clone)]
pub struct WindowProps {
  title: String,
  id: String,
  children: Element,
  header_icon: bool,
  footer_text: Option<String>,
  bounce: Option<Signal<bool>>,
  index: i16,
  extra_style: Option<String>,
  is_visible: Signal<bool>,
}

#[allow(non_snake_case)]
#[component]
pub fn WindowTemplate(props: WindowProps) -> Element {
  let mut div_element = use_signal(|| None as Option<Rc<MountedData>>);

  let mut size_style = use_signal(|| "".to_string());

  use_effect(move || {
    size_style.set("transform: none !important;".to_string());
  });

  // gross hack to allow use of props in multiple functions
  // let mut sig_taskbar_props = Signal::new(TaskbarItemProps {
  //   id: props.id.clone(),
  //   title: props.title.clone(),
  //   icon: None,
  //   is_visible: Signal::new(true) // should never be used
  // });

  let id_clone = Signal::new(props.id);

  let mut is_visible = props.is_visible;

  // TODO: update linter script to not do this
  let bouncing = if let Some(b) = props.bounce {
    b()
  } else {
    false
  };

  let radio_state = use_context::<RadioState>();
  let mut taskbar_items = radio_state.taskbar_items;
  let drag_state = radio_state.drag_state;

  let mut is_dragging = drag_state.is_dragging;
  let mut active_window = drag_state.active_window;

  let is_active = active_window() == id_clone();
  let window_index = if is_active { 100 } else { props.index };

  let mut dim_x = drag_state.dim_x;
  let mut dim_y = drag_state.dim_y;

  let mut dim_x_local = use_signal(|| String::from("50%"));
  let mut dim_y_local = use_signal(|| String::from("50%"));

  let mut previous_x = drag_state.previous_x;
  let mut previous_y = drag_state.previous_y;

  let read_dims = move || async move {
    let read = div_element.read();
    let client_rect = read.as_ref().map(|el| el.get_client_rect());

    if let Some(client_rect) = client_rect {
      if let Ok(rect) = client_rect.await {
        let diff_x = (rect.max_x() + rect.min_x()) / 2.0;
        let diff_y = (rect.max_y() + rect.min_y()) / 2.0;

        dim_x.set(diff_x);
        dim_y.set(diff_y);
      }
    }
  };

  rsx! {
    div {
      id: "{id_clone}",
      class: if bouncing { "window bouncing" } else { "window" },
      onmounted: move |cx| {
        div_element.set(Some(cx.data()));
        // if taskbar_items.iter().find(|item| item.id == sig_taskbar_props().id ).is_none() { taskbar_items.push(TaskbarItemProps { id: props.id.clone(), title: props.title.clone(), is_visible: is_visible, icon: None }); };
      },
      onmousedown: move |_| active_window.set(id_clone()),
      onmouseup: move |_| is_dragging.set(false),
      onmousemove: move |_| {
        if is_active && is_dragging() {
          dim_x_local.set(format!("{}px", dim_x()));
          dim_y_local.set(format!("{}px", dim_y()));
        }
      },
      style: if is_visible() {
          "display: auto;"
      } else {
          "display: none;"
      },
      style: if let Some(ref extra) = props.extra_style { "{extra}" },
      style: "z-index: {window_index};",
      style: if dim_x() > 0.0 && is_active && is_dragging() {"top: {dim_y}px; left: {dim_x}px;"} else {"top: {dim_y_local}; left: {dim_x_local};"},
      div {
        class: "inner",
        div {
          class: "header",
          onmousedown: move |_| {
            is_dragging.set(true);
            read_dims()
          },
          onmouseup: move |_| {
            is_dragging.set(false);
            previous_x.set(0.0);
            previous_y.set(0.0);
            dim_x_local.set(format!("{}px", dim_x()));
            dim_y_local.set(format!("{}px", dim_y()));
          },
          if props.header_icon {
            div { class: "icon", style: format!("background: url({}) no-repeat; background-size: cover;", ICON_FAVICON.to_string()) }
          },
          "{props.title}",
          div {
            class: "buttons",
            button {
              onclick: move |_| {
                is_visible.set(false);
              },
              class: "button-minimize",
              style: format!("background-image: url({});", ICON_CLOSE.to_string())
            }
          }
        },
        {props.children}
      },
      div {
        class: "status-bar",
        style: format!("background: url({});", RESIZE_ICON.to_string()),
        div { if let Some(foot) = props.footer_text { {foot} } else { {"Keep it Based."} } },
      }
    }
  }
}
