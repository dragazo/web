use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdMail;
use dioxus_free_icons::Icon;

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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Devin Jean" }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            id: "header",
            div {
                class: "container",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::Publications {}, "Publications" }
            }
        }
        div {
            id: "content",
            div {
                class: "container",
                Outlet::<Route> {}
            }
        }
        div {
            id: "footer",
            div {
                class: "container",
                small { "© 2026 Devin Jean" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        // Icon { width: 30, height: 30, fill: "black", icon: LdMail }
        div {
            class: "card row",
            div {
                class: "profile",
                img { src: asset!("/assets/profile.png") }
                h2 { "Devin Jean" }
            }
            div {
                h2 { "Contact" }
                a { href: "mailto:devin.jean@mtsu.edu",  "Email" }
            }
        }
        a { href: "https://github.com/dragazo", "Github" }
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
