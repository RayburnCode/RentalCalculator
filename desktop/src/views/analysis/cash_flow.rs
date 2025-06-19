use crate::common::*;


#[component]
pub fn CashFlow() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "cash-flow",
            p { "" }
            // After all attributes are defined, we can define child elements and components
            h1 { class: "text-4xl font-bold mb-4", "Cash Flow Analysis" }
            p { class: "text-lg text-gray-700",
                "This section will help you analyze the cash flow of your property."
            }
            p { class: "text-lg text-gray-700",
                "You can input your income and expenses to see your cash flow."
            }
            p { class: "text-lg text-gray-700",
                "This will help you make informed decisions about your property."
            }
            p { class: "text-lg text-gray-700",
                "You can also compare your cash flow with other properties."
            }
            p { class: "text-lg text-gray-700",
                "This will help you identify potential investment opportunities."
            }
        }
    }
}
