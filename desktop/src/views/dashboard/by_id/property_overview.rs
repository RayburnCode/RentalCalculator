use dioxus::prelude::*;
use crate::PropertyData;


/// Property Overview Tab
#[component]
fn PropertyOverview(property: PropertyData) -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-semibold mb-4", "Property Details" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div {
                    h3 { class: "font-bold", "Address" }
                    p { {property.address.clone()} }
                }
                div {
                    h3 { class: "font-bold", "Purchase Price" }
                    p { {format!("${:.2}", property.purchase_price)} }
                }
                        // Add more property details as needed
            }
            // Add map or image section if available
            div { class: "mt-6",
                h3 { class: "font-bold mb-2", "Location" }
                div { class: "bg-gray-200 h-64 rounded",
                    "Map placeholder (would show property location)"
                }
            }
        }
    }
}
