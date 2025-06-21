use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PropertyData {
    id: u32,
    address: String,
    purchase_price: f64,
    // Add more fields as needed
}

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

/// Financial Calculations Tab
#[component]
fn FinancialCalculations(property: PropertyData) -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-semibold mb-4", "Financial Analysis" }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                div {
                    h3 { class: "font-bold", "Purchase Details" }
                    div { class: "space-y-2",
                        p { {format!("Price: ${:.2}", property.purchase_price)} }
                        p { {format!("Down Payment: ${:.2}", 50000.0)} } // Example
                        p { {format!("Loan Amount: ${:.2}", 200000.0)} } // Example
                    }
                }
                div {
                    h3 { class: "font-bold", "Income" }
                    div { class: "space-y-2",
                        p { {format!("Monthly Rent: ${:.2}", 2500.0)} } // Example
                        p { {format!("Annual Income: ${:.2}", 30000.0)} } // Example
                    }
                }
                div {
                    h3 { class: "font-bold", "Expenses" }
                    div { class: "space-y-2",
                        p { {format!("Taxes: ${:.2}/yr", 3000.0)} } // Example
                        p { {format!("Insurance: ${:.2}/yr", 1200.0)} } // Example
                        p { {format!("Maintenance: ${:.2}/yr", 1500.0)} } // Example
                    }
                }
            }
            div { class: "mt-6",
                h3 { class: "font-bold mb-2", "ROI Calculations" }
                table { class: "w-full border-collapse",
                    thead {
                        tr { class: "bg-gray-100",
                            th { class: "p-2 text-left border", "Metric" }
                            th { class: "p-2 text-left border", "Value" }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "p-2 border", "Cash Flow" }
                            // Example
                            td { class: "p-2 border", {format!("${:.2}/yr", 12300.0)} }
                        }
                        tr {
                            td { class: "p-2 border", "Cap Rate" }
                            td { class: "p-2 border", {format!("{:.1}%", 6.5)} } // Example
                        }
                        tr {
                            td { class: "p-2 border", "Cash on Cash ROI" }
                            td { class: "p-2 border", {format!("{:.1}%", 12.3)} } // Example
                        }
                    }
                }
            }
        }
    }
}

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

/// Property Documents Tab
#[component]
fn PropertyDocuments(property: PropertyData) -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-semibold mb-4", "Property Documents" }
            div { class: "space-y-4",
                div {
                    h3 { class: "font-bold mb-2", "Upload New Document" }
                    input {
                        class: "block w-full text-sm text-gray-500
                                file:mr-4 file:py-2 file:px-4
                                file:rounded-md file:border-0
                                file:text-sm file:font-semibold
                                file:bg-blue-50 file:text-blue-700
                                hover:file:bg-blue-100",
                        r#type: "file",
                    }
                }
                div {
                    h3 { class: "font-bold mb-2", "Existing Documents" }
                    table { class: "w-full border-collapse",
                        thead {
                            tr { class: "bg-gray-100",
                                th { class: "p-2 text-left border", "Document" }
                                th { class: "p-2 text-left border", "Type" }
                                th { class: "p-2 text-left border", "Date" }
                                th { class: "p-2 text-left border", "Actions" }
                            }
                        }
                        tbody {
                            tr {
                                td { class: "p-2 border", "Purchase Agreement" }
                                td { class: "p-2 border", "Contract" }
                                td { class: "p-2 border", "2023-05-15" }
                                td { class: "p-2 border",
                                    button { class: "text-blue-500 hover:underline", "View" }
                                    " | "
                                    button { class: "text-blue-500 hover:underline", "Download" }
                                }
                            }
                            tr {
                                td { class: "p-2 border", "Inspection Report" }
                                td { class: "p-2 border", "Inspection" }
                                td { class: "p-2 border", "2023-05-20" }
                                td { class: "p-2 border",
                                    button { class: "text-blue-500 hover:underline", "View" }
                                    " | "
                                    button { class: "text-blue-500 hover:underline", "Download" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}