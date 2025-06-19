use crate::common::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn AddProperty(id: u32) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100",
            h1 { class: "text-4xl font-bold mb-4", "Add Property" }
            p { class: "text-lg text-gray-700", "Form to add a new property will be displayed here." }
        }
        h1 { "This is Property #{id}!" }
    }
}
