use dioxus::prelude::*;

mod api;
mod components;

pub use crate::api::*;
pub use crate::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const AVATAR: Asset = asset!("/assets/avatar.png");
const ACE_FLAG: Asset = asset!("/assets/ace_flag.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}

    }
}

#[component]
fn Home() -> Element {
    let tab = use_signal(|| String::from("Home"));

    rsx! {
        main { class: "absolute top-1/2 left-1/2 -translate-1/2 w-200",
            TabGroup {
                Tab { name: "Home", selection: tab }
                Tab { name: "Projects", selection: tab }
                Tab { name: "Contact", selection: tab }
                Tab { name: "Support", selection: tab }
            }
            div { class: "bg-surface border-2 border-surface-1 rounded-lg p-2 mt-2",
                if tab() == "Home" {
                    div { class: "flex w-full gap-2",
                        img { src: AVATAR, class: "w-25 rounded-full" },
                        div { class: "h-full flex flex-col justify-center",
                            h1 { "Drew" }
                            Subtext { "He/Him" }
                            img { src: ACE_FLAG, class: "w-10" }
                        }
                    }

                    Divider {}

                    p { "Hi! I'm a computer science major from Colorado and I like building useful things on the internet!" }
                    p { "I make projects mainly for myself, with hopes that other people will also find them useful!" }
                    p { "All of my projects are made without AI. I enjoy the process of programming and not destroying the environment!" }
                    p { "I've also contributed to a couple open-source projects." }
                    p { "If you have any questions, feel free to use the Contact page or email me!" }
                 }
                 if tab() == "Projects" {
                    div { class: "grid grid-cols-2 gap-2",
                        Project { user: "MrDiamondDog", repo: "Filatrack" }
                    }
                 }
            }
        }
    }
}
