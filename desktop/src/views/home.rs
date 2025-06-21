use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-blue-50 to-white",
            // Hero Section
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20",
                div { class: "text-center",
                    h1 { class: "text-4xl md:text-6xl font-bold text-gray-900 mb-6",
                        span { class: "text-blue-600", "Analyze" }
                        " Rental Properties Like a Pro"
                    }
                    p { class: "text-xl text-gray-600 max-w-3xl mx-auto mb-10",
                        "Instantly calculate cash flow, cap rates, and ROI for any property. Make smarter investment decisions in seconds."
                    }
                    div { class: "flex flex-col sm:flex-row justify-center gap-4",

                        Link {
                            to: Route::Dashboard {},
                            class: "px-8 py-4 bg-white text-gray-800 font-medium rounded-lg hover:bg-gray-100 border border-gray-300 transition text-lg",
                            "View Your Portfolio"
                        }
                    }
                }
            }

            // Value Proposition Cards
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                h2 { class: "text-3xl font-bold text-center text-gray-900 mb-12",
                    "Why Real Estate Investors Love Us"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    ValueCard {
                        icon: "M9 17v-2m3 2v-4m3 4v-6m2 10H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5.586a1 1 0 0 1 .707.293l5.414 5.414a1 1 0 0 1 .293.707V19a2 2 0 0 1-2 2z",
                        title: "Instant Analysis",
                        description: "Get key metrics like cap rate and cash flow in seconds with our optimized calculator.",
                    }
                    ValueCard {
                        icon: "M12 8v4l3 3m6-3a9 9 0 1 1-18 0 9 9 0 0 1 18 0z",
                        title: "Save Hours",
                        description: "No more spreadsheets - we automate all calculations so you can evaluate more deals.",
                    }
                    ValueCard {
                        icon: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0 1 12 2.944a11.955 11.955 0 0 1-8.618 3.04A12.02 12.02 0 0 0 3 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
                        title: "Avoid Bad Deals",
                        description: "Our system flags warning signs so you don't overpay for properties.",
                    }
                }
            }

            // Calculator Preview Section
            div { class: "bg-white py-16",
                div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "text-center mb-12",
                        h2 { class: "text-3xl font-bold text-gray-900 mb-4", "See the Power in Action" }
                        p { class: "text-xl text-gray-600",
                            "Here's what you'll get with every analysis:"
                        }
                    }
                    div { class: "bg-gray-50 p-6 rounded-xl border border-gray-200",
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mb-6",
                            div { class: "space-y-4",
                                PreviewItem {
                                    label: "Purchase Price",
                                    value: "$350,000",
                                }
                                PreviewItem { label: "Monthly Rent", value: "$2,800" }
                                PreviewItem { label: "Cap Rate", value: "8.2%" }
                            }
                            div { class: "space-y-4",
                                PreviewItem { label: "Cash Flow", value: "$1,200/mo" }
                                PreviewItem { label: "Cash-on-Cash ROI", value: "12.5%" }
                                PreviewItem { label: "Break-Even", value: "2.7 years" }
                            }
                        }
                        div { class: "text-center" }
                    }
                }
            }

            // Testimonial Section
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20",
                div { class: "bg-blue-600 rounded-2xl p-8 md:p-12 text-white",
                    div { class: "max-w-3xl mx-auto text-center",
                        svg {
                            class: "h-12 w-12 text-blue-300 mx-auto mb-6",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path { d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" }
                        }
                        p { class: "text-xl md:text-2xl font-medium mb-6",
                            "This calculator helped me identify a 12% ROI property I almost passed on. I've analyzed 50+ deals here before buying my last two rentals!"
                        }
                        div { class: "font-bold", "Sarah K." }
                        div { class: "text-blue-200 text-sm", "Real Estate Investor, Austin TX" }
                    }
                }
            }
        }
    }
}

// ===== Supporting Components =====
#[component]
fn ValueCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "bg-white p-8 rounded-xl shadow-md hover:shadow-lg transition",
            svg {
                class: "h-12 w-12 text-blue-600 mb-4",
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
            h3 { class: "text-xl font-bold text-gray-900 mb-3", title }
            p { class: "text-gray-600", {description} }
        }
    }
}

#[component]
fn PreviewItem(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div { class: "flex justify-between items-center py-2 border-b border-gray-100",
            span { class: "text-gray-600", {label} }
            span { class: "font-medium text-gray-900", {value} }
        }
    }
}