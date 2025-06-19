use crate::components::{Echo, Hero};
use crate::common::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Income() -> Element {
    rsx! {
        p { "Property Income" }
        p { "Monthly Rent" }
        p { "Other Income" }
        p { "Total Income" }
        p { "Annual Income" }
    }
}
