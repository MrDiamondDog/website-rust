use dioxus::prelude::*;

#[component]
pub fn TabGroup(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row gap-2 justify-center items-center",
            {children}
        }
    }
}

#[component]
pub fn Tab(name: String, selection: Signal<String>) -> Element {
    let name = use_signal(|| name.clone());

    rsx! {
        div {
            class: "bg-surface p-2 text-center border-2 border-surface-1 rounded-lg cursor-pointer transition-all",
            class: if selection() == name() { "border-b-primary w-full" } else { "w-1/3" },
            onclick: move |_| selection.set(name()),
            "{name}"
        }
    }
}
