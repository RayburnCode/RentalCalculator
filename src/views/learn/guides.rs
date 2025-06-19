use dioxus::prelude::*;

#[component]
pub fn Guides() -> Element {

    rsx! {
        div { class: "p-4",
            h1 { class: "text-2xl font-bold mb-4", "Guides" }
            p { class: "mt-2 text-gray-600",
                "Explore our comprehensive guides to real estate investing."
            }
        }
    }
}