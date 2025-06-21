use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PropertyData {
    id: u32,
    address: String,
    purchase_price: f64,
    // Add more fields as needed
}

#[derive(Clone, Default)]
struct PropertyFormData {
    address: String,
    city: String,
    state: String,
    zip_code: String,
    property_type: String,
    purchase_price: String,
    square_feet: String,
    bedrooms: String,
    bathrooms: String,
    year_built: String,
    description: String,
}

/// Component for adding a new property
#[component]
pub fn AddProperty(id: u32) -> Element {
    let mut form_data = use_signal(PropertyFormData::default);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    let submit_form = move |evt: FormEvent| {
        evt.prevent_default();

        // Validate form data
        if form_data.read().address.is_empty() {
            error_message.set("Address is required".to_string());
            return;
        }

        if form_data.read().purchase_price.parse::<f64>().is_err() {
            error_message.set("Purchase price must be a valid number".to_string());
            return;
        }

        // Clear any previous messages
        error_message.set(String::new());
        
        // Here you would typically:
        // 1. Process the form data
        // 2. Send to your backend API
        // 3. Handle the response
        
        // For now, just show a success message
        success_message.set(format!(
            "Property at {} added successfully!",
            form_data.read().address.clone()
        ));
        
        // Reset form if needed
        // form_data.set(PropertyFormData::default());
    };

    rsx! {
        div { class: "container mx-auto p-4 max-w-4xl",
            if !error_message.read().is_empty() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    {error_message.read().clone()}
                }
            }
            if !success_message.read().is_empty() {
                div { class: "bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4",
                    {success_message.read().clone()}
                }
            }

            div { class: "bg-white p-6 rounded-lg shadow",
                form { onsubmit: submit_form, prevent_default: "onsubmit",

                    // Address Section
                    div { class: "mb-6",
                        h2 { class: "text-xl font-semibold mb-4 border-b pb-2", "Address Information" }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            // Address
                            div {
                                label { class: "block text-gray-700 mb-2", "Street Address" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "123 Main St",
                                    value: "{form_data.read().address}",
                                    oninput: move |e| form_data.write().address = e.value(),
                                    required: true,
                                }
                            }
                            // City
                            div {
                                label { class: "block text-gray-700 mb-2", "City" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "Anytown",
                                    value: "{form_data.read().city}",
                                    oninput: move |e| form_data.write().city = e.value(),
                                }
                            }
                            // State
                            div {
                                label { class: "block text-gray-700 mb-2", "State" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "CA",
                                    value: "{form_data.read().state}",
                                    oninput: move |e| form_data.write().state = e.value(),
                                }
                            }
                            // Zip Code
                            div {
                                label { class: "block text-gray-700 mb-2", "ZIP Code" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "12345",
                                    value: "{form_data.read().zip_code}",
                                    oninput: move |e| form_data.write().zip_code = e.value(),
                                }
                            }
                        }
                    }

                    // Property Details Section
                    div { class: "mb-6",
                        h2 { class: "text-xl font-semibold mb-4 border-b pb-2", "Property Details" }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            // Property Type
                            div {
                                label { class: "block text-gray-700 mb-2", "Property Type" }
                                select {
                                    class: "w-full p-2 border rounded",
                                    value: "{form_data.read().property_type}",
                                    oninput: move |e| form_data.write().property_type = e.value(),
                                    option { value: "", "Select property type" }
                                    option { value: "Single Family", "Single Family Home" }
                                    option { value: "Multi Family", "Multi-Family Home" }
                                    option { value: "Condo", "Condominium" }
                                    option { value: "Townhouse", "Townhouse" }
                                    option { value: "Apartment", "Apartment Building" }
                                    option { value: "Commercial", "Commercial Property" }
                                }
                            }
                            // Purchase Price
                            div {
                                label { class: "block text-gray-700 mb-2", "Purchase Price" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "250000",
                                    value: "{form_data.read().purchase_price}",
                                    oninput: move |e| form_data.write().purchase_price = e.value(),
                                    step: "0.01",
                                    min: "0",
                                }
                            }
                            // Square Feet
                            div {
                                label { class: "block text-gray-700 mb-2", "Square Feet" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "1500",
                                    value: "{form_data.read().square_feet}",
                                    oninput: move |e| form_data.write().square_feet = e.value(),
                                    min: "0",
                                }
                            }
                            // Year Built
                            div {
                                label { class: "block text-gray-700 mb-2", "Year Built" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "1990",
                                    value: "{form_data.read().year_built}",
                                    oninput: move |e| form_data.write().year_built = e.value(),
                                    min: "1800",
                                    max: "2025",
                                }
                            }
                            // Bedrooms
                            div {
                                label { class: "block text-gray-700 mb-2", "Bedrooms" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "3",
                                    value: "{form_data.read().bedrooms}",
                                    oninput: move |e| form_data.write().bedrooms = e.value(),
                                    min: "0",
                                    step: "1",
                                }
                            }
                            // Bathrooms
                            div {
                                label { class: "block text-gray-700 mb-2", "Bathrooms" }
                                input {
                                    class: "w-full p-2 border rounded",
                                    placeholder: "2",
                                    value: "{form_data.read().bathrooms}",
                                    oninput: move |e| form_data.write().bathrooms = e.value(),
                                    min: "0",
                                    step: "0.5",
                                }
                            }
                        }
                    }
                    // Description Section
                    div { class: "mb-6",
                        h2 { class: "text-xl font-semibold mb-4 border-b pb-2", "Description" }
                        textarea {
                            class: "w-full p-2 border rounded",
                            placeholder: "Enter any additional details about the property...",
                            value: "{form_data.read().description}",
                            oninput: move |e| form_data.write().description = e.value(),
                        }
                    }
                    // Submit Button
                    div { class: "mt-6",
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            r#type: "submit",
                            "Add Property"
                        }
                    }
                }
            }

            // Optional: Add image upload section
            div { class: "bg-white p-6 rounded-lg shadow mt-6",
                h2 { class: "text-xl font-semibold mb-4", "Property Images" }
                input {
                    class: "block w-full text-sm text-gray-500
                            file:mr-4 file:py-2 file:px-4
                            file:rounded-md file:border-0
                            file:text-sm file:font-semibold
                            file:bg-blue-50 file:text-blue-700
                            hover:file:bg-blue-100",
                    r#type: "file",
                    accept: "image/*",
                    multiple: true,
                }
                p { class: "text-sm text-gray-500 mt-2", "Upload images of the property (max 10)" }
            }
        }
    }
}