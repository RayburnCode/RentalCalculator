use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);

    rsx! {
        nav { class: "bg-white shadow-lg",
            div { class: "max-w-6xl mx-auto px-4",
                div { class: "flex justify-between",
                    // Logo/Brand (Left side)
                    div { class: "flex space-x-4",
                        div { class: "flex items-center py-4 px-2",
                            Link {
                                to: Route::Home {},
                                class: "text-xl font-semibold text-gray-800 hover:text-blue-600",
                                "RentalCalc"
                            }
                        }
                    }

                    // Primary Nav (Center - Desktop)
                    div { class: "hidden md:flex items-center space-x-1",
                        NavLink { to: Route::Home {}, "Home" }
                        NavLink { to: Route::Dashboard {}, "Dashboard" }
                        NavLink { to: Route::TestingServer {}, "Testing Server" }
                    }

                    // Mobile Menu Button (Right side)
                    div { class: "md:hidden flex items-center",
                        button {
                            class: "outline-none mobile-menu-button",
                            onclick: move |_| is_mobile_menu_open.toggle(),
                            svg {
                                class: "w-6 h-6 text-gray-800",
                                fill: "none",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path { d: if is_mobile_menu_open() { "M6 18L18 6M6 6l12 12" } else { "M4 6h16M4 12h16M4 18h16" } }
                            }
                        }
                    }
                }
            }

            // Mobile Menu (Dropdown)
            div { class: format!("md:hidden {}", if is_mobile_menu_open() { "block" } else { "hidden" }),
                div { class: "px-2 pt-2 pb-3 space-y-1 sm:px-3",
                    MobileNavLink { to: Route::Home {}, "Home" }
                }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn NavLink(to: Route, children: Element) -> Element {
    rsx! {
        Link {
            to: to.clone(),
            class: "py-4 px-3 text-gray-800 hover:text-blue-600 font-medium",
            active_class: "text-blue-600 border-b-2 border-blue-600",
            children,
        }
    }
}

#[component]
fn MobileNavLink(to: Route, children: Element) -> Element {
    rsx! {
        Link {
            to: to.clone(),
            class: "block py-2 px-3 text-gray-800 hover:bg-gray-100 rounded",
            active_class: "bg-blue-100 text-blue-600",
            children,
        }
    }
}