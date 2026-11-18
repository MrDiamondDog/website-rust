use dioxus::prelude::*;

use crate::components::Btn;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "absolute top-1/2 left-1/2 -translate-1/2",
            img { src: "http://http.cat/404.jpg" }
            Btn { href: "/", "Home" }
        }
    }
}
