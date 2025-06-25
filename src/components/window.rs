use dioxus::html::input_data::MouseButton;
use dioxus::logger::tracing::info;
use dioxus::{
    html::geometry::euclid::{Point2D, Rect, Vector2D},
    prelude::*,
};
use std::rc::Rc;

static ICON_CLOSE: Asset = asset!("/assets/ui/element2.png");
static ICON_FAVICON: Asset = asset!("/assets/icons/favicon-32x32.png");

#[derive(PartialEq, Props, Clone)]
pub struct WindowProps {
    title: String,
    id: String,
    children: Element,
    headerIcon: bool
}

#[component]
pub fn Window(props: WindowProps) -> Element {
    let mut div_element = use_signal(|| None as Option<Rc<MountedData>>);
    let mut is_dragging = use_signal(|| false);
    let mut previous_x = use_signal(|| 0 as f32);
    let mut previous_y = use_signal(|| 0 as f32);

    let mut dim_x = use_signal(|| String::from("50%"));
    let mut dim_y = use_signal(|| String::from("50%"));

    let read_dims = move || async move {
        let read = div_element.read();
        let client_rect = read.as_ref().map(|el| el.get_client_rect());

        if let Some(client_rect) = client_rect {
            if let Ok(rect) = client_rect.await {
                let diff_x = (rect.max_x() as f32 + rect.min_x() as f32) / 2.0;
                let diff_y = (rect.max_y() as f32 + rect.min_y() as f32) / 2.0;

                dim_x.set(format!("{:?}", diff_x));
                dim_y.set(format!("{:?}", diff_y));
            }
        }
    };

    // TODO: don't let the window move out of bounds
    let mouse_move = move |event: Event<MouseData>| async move {
        if event.held_buttons().contains(MouseButton::Primary) && is_dragging() {
            // current mouse pos
            let screen_coords = event.screen_coordinates();
            // set previous to current if new
            if previous_x() == 0.0 {
                previous_x.set(screen_coords.x as f32)
            }
            if previous_y() == 0.0 {
                previous_y.set(screen_coords.y as f32)
            }

            let offset_x = previous_x() - screen_coords.x as f32;
            let offset_y = previous_y() - screen_coords.y as f32;

            let new_x = (dim_x().replace("%", "").parse::<f32>().unwrap() - offset_x).abs();
            let new_y = (dim_y().replace("%", "").parse::<f32>().unwrap() - offset_y).abs();

            dim_x.set(format!("{:?}", new_x));
            dim_y.set(format!("{:?}", new_y));

            // Finally, update the previous coords to the current pos
            previous_x.set(screen_coords.x as f32);
            previous_y.set(screen_coords.y as f32);
        }
    };

    rsx! {
        div {
            id: "{props.id}",
            class: "window",
            onmounted: move |cx| div_element.set(Some(cx.data())),
            style: "top: {dim_y}px; left: {dim_x}px;",
            div {
                class: "inner",
                div {
                    class: "header",
                    onmousedown: move |_| {
                        is_dragging.set(true);
                        read_dims()
                    },
                    onmouseup: move |_| { info!("mouseup!!"); is_dragging.set(false) },
                    onmousemove: move |event| mouse_move(event),
                    onmouseleave: move |event| mouse_move(event),
                    onmouseout: move |event| mouse_move(event),
                    div { class: "icon", style: format!("background: url({}) no-repeat; background-size: cover;", ICON_FAVICON.to_string()) },
                    "{props.title}",
                    div {
                        class: "buttons",
                        button { class: "button-minimize", style: format!("background-image: url({});", ICON_CLOSE.to_string()) }
                    }
                },
                {props.children}
            },
            div {
                class: "player-footer",
                div { "Keep it Based." },
                div { class: "footer-end" }
            }
        }
    }
}
