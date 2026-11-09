use dioxus::prelude::*;

#[component]
pub fn Subtext(children: Element) -> Element {
    rsx! {
        p { class: "text-foreground-1 text-sm", {children} }
    }
}
