

// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use super::Footer;
use super::Navbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: " flex-1" }
            Footer {}
        }
    }
}