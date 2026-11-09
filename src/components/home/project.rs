use crate::api::*;
use dioxus::{fullstack::Loading, prelude::*};

#[component]
pub fn Project(user: String, repo: String) -> Element {
    let github_res = use_loader(move || get_repo(user.clone(), repo.clone()));

    rsx! {
        div { class: "bg-surface-2 p-2 rounded-lg border-2 border-surface-3 hover:border-primary",
            { match github_res {
                Ok(res) => {
                    let user = res.read();
                    rsx! {
                        div { class: "flex flex-row gap-2 items-center",
                            img { src: user.owner.avatar_url.clone(), class: "w-8 rounded-full" }
                            a { href: user.owner.html_url.clone(), target: "_blank", {user.owner.login.clone()} }
                         }
                        h2 { {user.name.clone() } }
                        p { {user.description.clone().unwrap_or_else(|| "".to_string())} }
                    }
                },
                Err(e) => match e {
                    Loading::Pending(_) => rsx! { "Loading..." },
                    Loading::Failed(_) => rsx! { "Error: {e}" }
                },
            }}
        }
    }
}
