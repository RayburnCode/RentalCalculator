use dioxus::prelude::*;

#[component]
pub fn Calculator() -> Element {
    let mut purchase_price = use_signal(|| 300_000.0);
    let mut down_payment_percent = use_signal(|| 20.0);
    let mut interest_rate = use_signal(|| 6.0);
    let mut loan_term = use_signal(|| 30);
    let mut monthly_rent = use_signal(|| 2_000.0);
    let mut vacancy_rate = use_signal(|| 5.0);
    let mut property_taxes = use_signal(|| 3_000.0);
    let mut insurance = use_signal(|| 1_200.0);
    let mut maintenance_percent = use_signal(|| 5.0);
    let mut property_management_percent = use_signal(|| 8.0);
    let mut hoa_fees = use_signal(|| 0.0);

    let down_payment = purchase_price() * (down_payment_percent() / 100.0);
    let loan_amount = purchase_price() - down_payment;

    // Monthly mortgage calculation
    let monthly_interest_rate = interest_rate() / 100.0 / 12.0;
    let loan_term_months = loan_term() * 12;
    let monthly_mortgage = if loan_term_months > 0 && monthly_interest_rate > 0.0 {
        (loan_amount * monthly_interest_rate * (1.0f64 + monthly_interest_rate).powi(loan_term_months))
            / ((1.0f64 + monthly_interest_rate).powi(loan_term_months) - 1.0f64)
    } else {
        0.0
    };

    // Expenses
    let monthly_taxes = property_taxes() / 12.0;
    let monthly_insurance = insurance() / 12.0;
    let monthly_maintenance = monthly_rent() * (maintenance_percent() / 100.0);
    let monthly_management = monthly_rent() * (property_management_percent() / 100.0);
    let vacancy_loss = monthly_rent() * (vacancy_rate() / 100.0);

    let total_monthly_expenses =
        monthly_taxes + monthly_insurance + monthly_maintenance + monthly_management + hoa_fees();
    let net_operating_income = monthly_rent() - vacancy_loss - total_monthly_expenses;
    let cash_flow = net_operating_income - monthly_mortgage;

    // ROI & Cap Rate
    let annual_cash_flow = cash_flow * 12.0;
    let cap_rate = (net_operating_income * 12.0) / purchase_price() * 100.0;
    let cash_on_cash_return = (annual_cash_flow / down_payment) * 100.0;

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 bg-white rounded-lg shadow-md",
            h1 { class: "text-2xl font-bold text-gray-800 mb-6", "Rental Property Calculator" }

            // Input Section
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mb-8",
                // Property Details
                div { class: "space-y-4",
                    h2 { class: "text-xl font-semibold text-gray-700", "Property Details" }
                    InputField {
                        label: "Purchase Price ($)",
                        value: purchase_price(),
                        on_change: move |v| purchase_price.set(v),
                    }
                    InputField {
                        label: "Down Payment (%)",
                        value: down_payment_percent(),
                        on_change: move |v| down_payment_percent.set(v),
                    }
                    InputField {
                        label: "Interest Rate (%)",
                        value: interest_rate(),
                        on_change: move |v| interest_rate.set(v),
                    }
                    InputField {
                        label: "Loan Term (years)",
                        value: loan_term() as f64,
                        on_change: move |v| loan_term.set(v as i32),
                    }
                }

                // Rental Income & Expenses
                div { class: "space-y-4",
                    h2 { class: "text-xl font-semibold text-gray-700", "Rental Income & Expenses" }
                    InputField {
                        label: "Monthly Rent ($)",
                        value: monthly_rent(),
                        on_change: move |v| monthly_rent.set(v),
                    }
                    InputField {
                        label: "Vacancy Rate (%)",
                        value: vacancy_rate(),
                        on_change: move |v| vacancy_rate.set(v),
                    }
                    InputField {
                        label: "Property Taxes ($/year)",
                        value: property_taxes(),
                        on_change: move |v| property_taxes.set(v),
                    }
                    InputField {
                        label: "Insurance ($/year)",
                        value: insurance(),
                        on_change: move |v| insurance.set(v),
                    }
                    InputField {
                        label: "Maintenance (%)",
                        value: maintenance_percent(),
                        on_change: move |v| maintenance_percent.set(v),
                    }
                    InputField {
                        label: "Property Management (%)",
                        value: property_management_percent(),
                        on_change: move |v| property_management_percent.set(v),
                    }
                    InputField {
                        label: "HOA Fees ($/month)",
                        value: hoa_fees(),
                        on_change: move |v| hoa_fees.set(v),
                    }
                }
            }

            // Results Section
            div { class: "bg-gray-50 p-6 rounded-lg",
                h2 { class: "text-xl font-semibold text-gray-700 mb-4", "Investment Summary" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "space-y-2",
                        ResultItem {
                            label: "Down Payment",
                            value: format!("${:.2}", down_payment),
                        }
                        ResultItem {
                            label: "Loan Amount",
                            value: format!("${:.2}", loan_amount),
                        }
                        ResultItem {
                            label: "Monthly Mortgage",
                            value: format!("${:.2}", monthly_mortgage),
                        }
                        ResultItem {
                            label: "Total Monthly Expenses",
                            value: format!("${:.2}", total_monthly_expenses),
                        }
                    }
                    div { class: "space-y-2",
                        ResultItem {
                            label: "Monthly Cash Flow",
                            value: format!("${:.2}", cash_flow),
                        }
                        ResultItem {
                            label: "Annual Cash Flow",
                            value: format!("${:.2}", annual_cash_flow),
                        }
                        ResultItem {
                            label: "Cap Rate (%)",
                            value: format!("{:.2}%", cap_rate),
                        }
                        ResultItem {
                            label: "Cash-on-Cash ROI (%)",
                            value: format!("{:.2}%", cash_on_cash_return),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputField(label: &'static str, value: f64, on_change: EventHandler<f64>) -> Element {
    rsx! {
        div { class: "space-y-1",
            label { class: "block text-sm font-medium text-gray-700", "{label}" }
            input {
                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                r#type: "number",
                value: "{value}",
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<f64>() {
                        on_change.call(val);
                    }
                },
            }
        }
    }
}

#[component]
fn ResultItem(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "flex justify-between",
            span { class: "text-gray-600", "{label}" }
            span { class: "font-medium", "{value}" }
        }
    }
}