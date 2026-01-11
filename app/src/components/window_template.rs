use crate::components::{ICON_FAVICON, RadioState, windows::AboutButton};
// use dioxus::logger::tracing::info;
use dioxus::{html::geometry::PixelsRect, prelude::*, warnings::Warning};
use std::rc::Rc;

static RESIZE_ICON: Asset = asset!("/assets/ui/resize.png");

#[derive(PartialEq, Props, Clone)]
pub struct WindowProps {
  title: String, // shown in the window header
  id: String,    // used internally for the taskbar
  children: Element,
  header_icon: Option<Asset>,
  footer_text: Option<String>,
  bounce: Option<Signal<bool>>, // only used in the main player window
  index: i16,                   // z-index
  extra_style: Option<String>,
  is_visible: Signal<bool>,
  extra_menu_btn: Option<Element>,
  // the offsets are css percentage based, beyond +/- 50 will be cut off
  x_offset: Option<i16>,
  y_offset: Option<i16>,
}

fn close_window(id: &str) {
  let mut open_windows = use_context::<RadioState>().open_windows;

  if let Some(inx) = open_windows.iter().position(|item| item.id == id) {
    open_windows.remove(inx);
  }
}

// Ran when the user grabs a corner of a window
// TODO:
// async fn resize_window(
//   event: Event<MouseData>,
//   element: Signal<Option<Rc<MountedData>>>,
// ) -> Option<(f64, f64)> {
//   let read = element.read();
//   let client_rect = read.as_ref().map(|el| el.get_client_rect());

//   let (dx, dy) = {
//     let coords = event.data.client_coordinates();
//     (coords.x as f64, coords.y as f64)
//   };

//   if let Some(client_rect) = client_rect {
//     if let Ok(rect) = client_rect.await {
//       return Some((
//         (rect.size.width + dx).max(50.0),
//         (rect.size.height + dy).max(50.0),
//       ));
//     }
//   }
//   return None;
// }

#[allow(non_snake_case)]
#[component]
pub fn WindowTemplate(mut props: WindowProps) -> Element {
  let mut div_element = use_signal(|| None as Option<Rc<MountedData>>);
  let mut is_visible = warnings::copy_value_hoisted::allow(|| props.is_visible);

  let vis_css = use_memo(move || {
    if warnings::copy_value_hoisted::allow(|| is_visible()) {
      "display: inline"
    } else {
      "display: none"
    }
  });

  let id_clone = Signal::new(props.id.clone());

  // TODO: update linter script to not do this
  let bouncing = if let Some(b) = props.bounce {
    b()
  } else {
    false
  };

  let mut window_height = Signal::new(None as Option<f64>);
  let mut window_width = Signal::new(None as Option<f64>);

  let radio_state = use_context::<RadioState>();
  let drag_state = radio_state.drag_state;

  let mut is_dragging = drag_state.is_dragging;
  let mut active_window = drag_state.active_window;

  let is_active = active_window() == id_clone();
  let window_index = if is_active { 100 } else { props.index };

  let mut dim_x = drag_state.dim_x;
  let mut dim_y = drag_state.dim_y;

  let mut dim_x_local =
    use_signal(|| String::from(format!("{:#?}%", 50 + props.x_offset.unwrap_or(0))));
  let mut dim_y_local =
    use_signal(|| String::from(format!("{:#?}%", 50 + props.y_offset.unwrap_or(0))));

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
      onmounted: move |cx| div_element.set(Some(cx.data())),
      onmousedown: move |_| active_window.set(id_clone()),
      onmouseup: move |_| is_dragging.set(false),
      onmousemove: move |_| {
        if is_active && is_dragging() {
          dim_x_local.set(format!("{}px", dim_x()));
          dim_y_local.set(format!("{}px", dim_y()));
        }
      },
      style: if let Some(ref extra) = props.extra_style { "{extra}" },
      style: "{vis_css}; z-index: {window_index};",
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
          if let Some(i) = props.header_icon {
            div { class: "icon", style: format!("background: url({}) no-repeat; background-size: cover;", i.to_string()) }
          },
          "{props.title}",
          div {
            class: "buttons",
            style: "flexbox",
            if &props.id == "based-radio" {
              AboutButton {  }
            },
            // Minimize button
            button {
              onclick: move |_| is_visible.set(false),
              class: "button-minimize",
              aria_label: "minimize-window",
            },
            // Close button
            if &props.id != "based-radio" {
              button {
                onclick: move |_| close_window(&props.id),
                class: "button-close",
                aria_label: "close-window",
              }
            } else { } // For whatever reason this only appears with this empty else
          }
        },
        {props.children}
      },
      div {
        class: "status-bar",
        style: format!("background: url({});", RESIZE_ICON.to_string()),
        // onmousedown: move |event| resize_window(event, div_element),
        div { { props.footer_text.unwrap_or(String::from("Keep it Based.")) } }
      }
    }
  }
}
