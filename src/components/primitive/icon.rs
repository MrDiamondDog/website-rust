use dioxus::prelude::*;

#[component]
pub fn Icon(icon: Asset, size: Option<u32>) -> Element {
    let size = match size {
        Some(val) => val,
        None => 20,
    };

    rsx! {
        // svg { width: size, height: size, class: "stroke-foreground",
        //     image { href: icon, width: size, height: size, class: "stroke-foreground" }
        // }
        img { src: icon, width: size, height: size, class: "stroke-foreground", role: "img" }
    }
}
