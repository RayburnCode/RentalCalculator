
use dioxus::prelude::*;

use views::{Blog, Home, AppLayout, PropertyDetails, };

mod components;
mod views;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    #[layout(AppLayout)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},

        #[route("/property-details")]
        PropertyDetails {},

        #[route("/blog/:id")]
        Blog { id: i32 },

            #[nest("/dashboard")]

            #[route("/")]
            Dashboard {},
            
            #[route("/:id")]
            Property { id: i32 },

        #[end_nest]

        #[end_layout]

        #[route("/not-found")]
        NotFound { route: Vec<String> },

}



const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
