use dioxus::prelude::*;



/// The Property Profile page with multiple tabs for analysis
#[component]
pub fn PropertyProfile(id: u32) -> Element {
    // State for the currently active tab
    let mut active_tab = use_signal(|| "overview");

    // Mock property data - in a real app, this would come from an API or database
    let property = PropertyData {
        id,
        address: "123 Main St, Anytown, USA".to_string(),
        purchase_price: 250000.0,
    };

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6", "Property Profile" }
            h2 { class: "text-xl mb-4", "Property #{id}" }

            // Tab navigation
            div { class: "flex border-b mb-6",
                TabButton {
                    active: *active_tab.read() == "overview",
                    onclick: move |_| active_tab.set("overview"),
                    label: "Overview",
                }
                TabButton {
                    active: *active_tab.read() == "calculations",
                    onclick: move |_| active_tab.set("calculations"),
                    label: "Financial Calculations",
                }
                TabButton {
                    active: *active_tab.read() == "conditions",
                    onclick: move |_| active_tab.set("conditions"),
                    label: "Conditions",
                }
                TabButton {
                    active: *active_tab.read() == "documents",
                    onclick: move |_| active_tab.set("documents"),
                    label: "Documents",
                }
            }

            // Tab content
            match *active_tab.read() {
                "overview" => rsx! {
                    PropertyOverview { property: property.clone() }
                },
                "calculations" => rsx! {
                    FinancialCalculations { property: property.clone() }
                },
                "conditions" => rsx! {
                    PropertyConditions { property: property.clone() }
                },
                "documents" => rsx! {
                    PropertyDocuments { property: property.clone() }
                },
                _ => rsx! {
                    div { "Invalid tab" }
                },
            }
        }
    }
}

/// Reusable tab button component
#[component]
fn TabButton(active: bool, onclick: EventHandler, label: String) -> Element {
    rsx! {
        button {
            class: if active { "px-4 py-2 font-medium text-sm rounded-t-lg bg-blue-500 text-white" } else { "px-4 py-2 font-medium text-sm rounded-t-lg hover:bg-gray-200" },
            onclick: move |_| {
                onclick.call(());
            },
            {label}
        }
    }
}



