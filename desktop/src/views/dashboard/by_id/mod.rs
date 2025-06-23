use dioxus::prelude::*;


mod property_overview;
pub use property_overview::PropertyOverview;


#[derive(PartialEq, Props, Clone)]
pub struct PropertyData {
    id: u32,
    address: String,
    purchase_price: f64,
}