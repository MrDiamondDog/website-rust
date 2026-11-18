use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum ButtonLooks {
    #[default]
    Primary,
    Secondary,
    Danger,
    Success,
}

#[component]
pub fn Btn(
    children: Element,
    href: Option<String>,
    target: Option<String>,
    onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)] look: ButtonLooks,
) -> Element {
    let buttonLook = match look {
        ButtonLooks::Primary => "bg-primary hover:bg-primary-1 active:bg-primary-2",
        ButtonLooks::Secondary => "bg-surface-1 hover:bg-surface-2 active:bg-primary-3",
        ButtonLooks::Danger => "bg-danger hover:bg-danger-1 active:bg-danger-2",
        ButtonLooks::Success => "bg-success hover:bg-success-1 active:bg-success-2",
        _ => "bg-primary hover:bg-primary-1 active:bg-primary-2",
    };

    rsx! {
        button { class: "rounded-lg hover:scale-[1.02] transition-all p-2 text-center w-full cursor-pointer {buttonLook}",
            onclick: move |e| {
                if !href.is_none() {
                    if !target.is_none() && target.clone().unwrap() == "_blank".to_string() {
                        document::eval(&format!("window.open(\"{}\")", href.clone().unwrap()).to_string());
                    } else {
                        navigator().push(href.clone().unwrap());
                    }
                }
                if !onclick.is_none() {
                    onclick.unwrap()(e);
                }
             },
            {children}
        }
    }
}
