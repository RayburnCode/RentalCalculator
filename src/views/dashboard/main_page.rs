use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Dashboard() -> Element {
    // Sample property data (replace with your state management)
    let properties = use_signal(|| vec![
        Property {
            id: 1,
            address: "123 Main St, Austin, TX".to_string(),
            purchase_price: 350_000.0,
            monthly_rent: 2_500.0,
            cap_rate: 7.2,
            status: PropertyStatus::Analyzing,
        },
        Property {
            id: 2,
            address: "456 Oak Ave, Denver, CO".to_string(),
            purchase_price: 475_000.0,
            monthly_rent: 3_200.0,
            cap_rate: 8.1,
            status: PropertyStatus::Interested,
        },
        Property {
            id: 3,
            address: "789 Beach Blvd, Miami, FL".to_string(),
            purchase_price: 620_000.0,
            monthly_rent: 4_100.0,
            cap_rate: 7.9,
            status: PropertyStatus::Rejected,
        },
    ]);

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-gray-900", "Property Dashboard" }
                p { class: "text-gray-600", "Track and analyze your potential investments" }
            }

            // Stats Cards
            div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 mb-8",
                StatCard {
                    title: "Properties Tracked",
                    value: properties().len().to_string(),
                    icon: "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5.586a1 1 0 0 1 .707.293l5.414 5.414a1 1 0 0 1 .293.707V19a2 2 0 0 1-2 2z",
                }
                StatCard {
                    title: "Avg Cap Rate",
                    value: format!(
                        "{:.1}%",
                        properties().iter().map(|p| p.cap_rate).sum::<f64>() / properties().len() as f64,
                    ),
                    icon: "M9 19v-6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2zm0 0V9a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v10m-6 0a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2m0 0V5a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2z",
                }
                StatCard {
                    title: "Total Value",
                    value: format!("${:.0}", properties().iter().map(|p| p.purchase_price).sum::<f64>()),
                    icon: "M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                }
                StatCard {
                    title: "Monthly Income",
                    value: format!("${:.0}", properties().iter().map(|p| p.monthly_rent).sum::<f64>()),
                    icon: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
                }
            }

            // Property Table
            div { class: "bg-white shadow rounded-lg overflow-hidden",
                div { class: "p-4 border-b flex justify-between items-center",
                    h2 { class: "text-lg font-medium", "Your Properties" }
                    Link {
                        to: Route::AddProperty { id: 0 }, // Update with your add property route
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition",
                        "Add New Property"
                    }
                }

                table { class: "min-w-full divide-y divide-gray-200",
                    thead { class: "bg-gray-50",
                        tr {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Address"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Purchase Price"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Monthly Rent"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Cap Rate"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Status"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Actions"
                            }
                        }
                    }
                    tbody { class: "bg-white divide-y divide-gray-200",
                        for property in properties().iter() {
                            tr { key: "{property.id}",
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    div { class: "text-sm font-medium text-gray-900",
                                        "{property.address}"
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    div { class: "text-sm text-gray-900",
                                        {format!("${:.0}", property.purchase_price)}
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    div { class: "text-sm text-gray-900",
                                        {format!("${:.0}", property.monthly_rent)}
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    div { class: "text-sm text-gray-900",
                                        {format!("{:.1}%", property.cap_rate)}
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    StatusBadge { status: property.status.clone() }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                    Link {
                                        to: Route::PropertyProfile { id: 1 }, // Update with your detail route
                                        class: "text-blue-600 hover:text-blue-900 mr-3",
                                        "View Details"
                                    }
                                    button {
                                        class: "text-red-600 hover:text-red-900",
                                        onclick: move |_| {},
                                        "Remove"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ===== Supporting Components =====
#[derive(Clone, PartialEq)]
struct Property {
    id: u32,
    address: String,
    purchase_price: f64,
    monthly_rent: f64,
    cap_rate: f64,
    status: PropertyStatus,
}

#[derive(Clone, PartialEq)]
enum PropertyStatus {
    Analyzing,
    Interested,
    Rejected,
}

#[component]
fn StatCard(title: &'static str, value: String, icon: &'static str) -> Element {
    rsx! {
        div { class: "bg-white p-4 rounded-lg shadow",
            div { class: "flex items-center",
                svg {
                    class: "h-8 w-8 text-blue-600 mr-3",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: icon,
                    }
                }
                div {
                    p { class: "text-sm font-medium text-gray-500", title }
                    p { class: "text-2xl font-semibold text-gray-900", "{value}" }
                }
            }
        }
    }
}

#[component]
fn StatusBadge(status: PropertyStatus) -> Element {
    let (class, text) = match status {
        PropertyStatus::Analyzing => ("bg-yellow-100 text-yellow-800", "Analyzing"),
        PropertyStatus::Interested => ("bg-green-100 text-green-800", "Interested"),
        PropertyStatus::Rejected => ("bg-red-100 text-red-800", "Rejected"),
    };

    rsx! {
        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full {class}",
            {text}
        }
    }
}