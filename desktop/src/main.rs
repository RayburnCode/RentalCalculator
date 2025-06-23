

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use views::{Home, AppLayout, NotFound, TestingServer};
use views::dashboard::{Dashboard, PropertyProfile, AddProperty};
use views::analysis::{Calculator, Comparison, Expenses, CashFlow};
use views::learn::{Guides, Glossary};
//mod components;
mod views;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/testing-server")]
        TestingServer {},
        
        // Property Analysis Routes
        #[route("/calculator")]
        Calculator {},
        
        #[route("/property/:id")]
        PropertyProfile { id: u32 },
        
        #[route("/compare")]
        Comparison {},
        
        // Financial Breakdown Routes
        #[nest("/finances")]
            #[route("/expenses")]
            Expenses {},
            
            #[route("/cashflow")]
            CashFlow {},
            
           // #[route("/tax-benefits")]
           // TaxBenefits {},
        #[end_nest]
        
        // Portfolio Management
        #[nest("/portfolio")]
            #[route("/dashboard")]
            Dashboard {},

            #[route("/add-property/:id")]
            AddProperty { id: u32 },
            
         //   #[route("/map-view")]
         //   PortfolioMap {},
            
          //  #[route("/performance")]
          //  PortfolioPerformance {},
        #[end_nest]
        
        // Market Data
       // #[route("/market-trends")]
       // MarketTrends {
        //    #[query]
         //   location: Option<String>,
     //   },
        
        // Educational Content
        #[nest("/learn")]
            #[route("/guides")]
            Guides {},
            
        //   #[route("/case-studies")]
        //    CaseStudies {},
            
            #[route("/glossary")]
            Glossary {},
        #[end_nest]
        
        // User Management
      //  #[route("/settings")]
     //   UserSettings {},
        
      //  #[route("/notifications")]
      //  Notifications {},
    #[end_layout]
    
 //   #[route("/login")]
 //   Login {},
    
    #[route("/not-found")]
    NotFound { route: Vec<String> },
}



const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Set the url of the server where server functions are hosted.
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8080");
    
    // Launch the desktop app with configuration
    dioxus::launch(app);
}



#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}


