use dioxus::prelude::*;

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
