use dioxus::prelude::*;

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