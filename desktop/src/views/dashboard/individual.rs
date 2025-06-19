use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn PropertyProfile(id: u32) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100",
            h1 { class: "text-4xl font-bold mb-4", "Property Profile" }
            p { class: "text-lg text-gray-700", "Details about the property will be displayed here." }
        }
        h1 { "This is Property #{id}!" }
    }
}
