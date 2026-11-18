use crate::{api::*, Icon, STAR_ICON};
use dioxus::prelude::*;

#[component]
pub fn Project(user: String, repo: String) -> Element {
    let user = use_signal(|| user.clone());
    let repo = use_signal(|| repo.clone());
    let github_res = use_resource(move || get_repo(user(), repo()));

    rsx! {
        a { class: "bg-surface-2 p-2 rounded-lg border-2 border-surface-3 hover:border-primary flex flex-col gap-2",
            href: "https://github.com/{user()}/{repo()}",
            target: "_blank",
            if let Some(res) = &*github_res.read() {
                match res {
                     Ok(repo) => {
                        rsx! {
                            div {
                                div { class: "flex flex-row gap-2 items-center",
                                    img { src: repo.owner.avatar_url.clone(), class: "w-8 rounded-full" }
                                    a { href: repo.owner.html_url.clone(), target: "_blank", {repo.owner.login.clone()} }
                                }
                                h2 { {repo.name.clone() } }
                                p { {repo.description.clone().unwrap_or_else(|| "".to_string())} }
                            }
                            div { class: "flex flex-row gap-2 items-center mt-auto",
                                Icon { icon: STAR_ICON }
                                {repo.stargazers_count.to_string()}
                            }
                        }
                    }
                    Err(e) => rsx! { "Error: {e}" }
                }
            } else {
                "Loading..."
            }
        }
    }
}
