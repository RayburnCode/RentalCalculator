use dioxus::prelude::*;


#[component]
pub fn Glossary() -> Element {

    rsx! {
        div { class: "p-4",
            h1 { class: "text-2xl font-bold mb-4", "Glossary of Terms" }
            ul {
                li { class: "mb-2", "Property: A piece of real estate, such as a house or apartment." }
                li { class: "mb-2",
                    "Investment: The act of putting money into property with the expectation of generating profit."
                }
                li { class: "mb-2",
                    "ROI (Return on Investment): A measure of the profitability of an investment, calculated as a percentage."
                }
                li { class: "mb-2",
                    "Cash Flow: The net amount of cash being transferred in and out of a property."
                }
                li { class: "mb-2",
                    "Expenses: Costs associated with owning and maintaining a property, such as repairs, taxes, and insurance."
                }
                li { class: "mb-2", "Income: Money earned from renting or leasing a property." }
                li { class: "mb-2",
                    "Market Trends: Patterns and changes in the real estate market that can affect property values."
                }
            }
            p { class: "mt-4 text-gray-600",
                "Understanding these terms is essential for making informed decisions in real estate investing."
            }
            p { class: "mt-2 text-gray-600",
                "For more detailed explanations, consider consulting a real estate professional or financial advisor."
            }
        }
    }
}