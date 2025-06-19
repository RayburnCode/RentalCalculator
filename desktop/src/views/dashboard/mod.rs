mod main_page;
mod individual;

use dioxus::html::mo;
pub use main_page::Dashboard;
pub use individual::PropertyProfile;

mod add_property;
pub use add_property::AddProperty;