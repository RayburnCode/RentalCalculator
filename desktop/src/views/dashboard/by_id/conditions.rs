use dioxus::prelude::*;

/// Property Conditions Tab
#[component]
fn PropertyConditions(property: PropertyData) -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-semibold mb-4", "Property Conditions" }
            div { class: "space-y-4",
                div {
                    h3 { class: "font-bold mb-2", "Inspection Items" }
                    ul { class: "list-disc pl-5",
                        li { "Roof needs replacement within 5 years" }
                        li { "HVAC system is 10 years old" }
                        li { "Foundation is in good condition" }
                    }
                }
                div {
                    h3 { class: "font-bold mb-2", "Required Repairs" }
                    ul { class: "list-disc pl-5",
                        li { "Fix leak in master bathroom" }
                        li { "Replace broken window in kitchen" }
                    }
                }
                div {
                    h3 { class: "font-bold mb-2", "Notes" }
                    textarea {
                        class: "w-full p-2 border rounded",
                        rows: "4",
                        placeholder: "Add any additional conditions or notes here...",
                    }
                }
            }
        }
    }
}
