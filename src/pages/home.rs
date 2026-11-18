use crate::assets::*;
use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let tab = use_signal(|| String::from("Home"));

    rsx! {
        main { class: "absolute top-1/2 left-1/2 -translate-1/2 w-200",
            TabGroup {
                Tab { name: "Home", selection: tab }
                Tab { name: "Projects", selection: tab }
                Tab { name: "Contact", selection: tab }
            }
            div { class: "bg-surface border-2 border-surface-1 rounded-lg p-2 mt-2",
                TabContent { visible: tab() == "Home",
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

                    Divider {}

                    div { class: "flex flex-row gap-2",
                        Btn { target: "_blank", href: "mailto:drew@drewrat.dev", "Email" }
                        Btn { target: "_blank", href: "https://discord.gg/rVp5BR8K96", "Discord" }
                        Btn { target: "_blank", href: "https://github.com/mrdiamonddog", "GitHub" }
                    }
                 }
                 TabContent { visible: tab() == "Projects",
                    div { class: "grid grid-cols-2 gap-2",
                        Project { user: "MrDiamondDog", repo: "Filatrack" }
                        Project { user: "MrDiamondDog", repo: "Balatrools" }
                        Project { user: "MrDiamondDog", repo: "Neptune" }
                        Project { user: "MrDiamondDog", repo: "Tungsten" }
                        Project { user: "MrDiamondDog", repo: "deltarunemusicdiscs" }
                        Project { user: "Vendicated", repo: "Vencord" }
                    }
                }
                TabContent { visible: tab() == "Contact",
                    form { action: "https://api.web3forms.com/submit", method: "POST",
                        input { type: "hidden", name: "access_key", value: "0ce3c452-83f4-420b-8ee1-97784e0029d4" }
                        input { type: "text", name: "name", required: true }
                        input { type: "email", name: "email", required: true }
                        textarea { name: "message", required: true }

                        Btn { "Submit" }
                    }
                }
            }
        }
    }
}
