use dioxus::prelude::*;

mod publications;
mod util;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/publications")]
    Publications {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Devin Jean" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Publications {}, "Publications" }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            id: "hero",
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

#[component]
fn Publications() -> Element {
    let pubs = publications::PUBLICATIONS.iter().map(|x| {
        let authors = util::pretty_join(&x.authors.iter().map(|&x| rsx! { if x == "Devin Jean" { strong { {x} } } else { {x} } }).collect::<Vec<_>>());
        rsx! { {authors} r#" ({x.year}), "{x.title}," "# em { {x.venue} } ". " a { href: x.link, {x.link} } }
    });
    rsx! {
        h1 { "Publications" }
        ol { for x in pubs { li { {x} } } }
    }
}
