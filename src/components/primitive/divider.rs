use dioxus::prelude::*;

#[component]
pub fn Divider(#[props(default)] vertical: bool) -> Element {
    rsx! {
        div {
            class: "bg-surface-1",
            class: if vertical { "self-stretch min-w-0.5 mx-2" } else { "w-full h-0.5 my-2" }
        }
    }
}
