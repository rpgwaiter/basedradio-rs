use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct WindowProps {
  title: String,
  children: Element
}

#[component]
pub fn Window(props: WindowProps) -> Element {
    rsx! {
        div {
            id: "player",
            div {
                id: "window-player",
                class: "win98",
                div {
                    id: "based-radio",
                    class: "window",
                    div {
                        class: "header",
                        div { class: "icon" },
                        "{props.title}",
                        div {
                            class: "buttons",
                            button { class: "button-minimize" }
                        }
                    },
                    {props.children}
                }
            }
        }
    }
}