use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdMail, LdGraduationCap};
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaOrcid};
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
                class: "container row buttons",
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
        div {
            class: "card row center fit",
            img { class: "profile", src: asset!("/assets/profile.png") }
            div {
                h2 { "Devin Jean" }
                p { "Assistant Professor" br {} "Department of Computer Science" br {} "Middle Tennessee State University" }
                h3 { "Contact" }
                div {
                    class: "row buttons",
                    a { href: "mailto:devin.jean@mtsu.edu", target: "_blank", Icon { width: 35, height: 35, icon: LdMail } }
                    a { href: "https://github.com/dragazo", target: "_blank", Icon { width: 30, height: 30, icon: FaGithub } }
                    a { href: "https://orcid.org/0000-0001-9549-2324", target: "_blank", Icon { width: 30, height: 30, icon: FaOrcid } }
                    a { href: "https://scholar.google.com/citations?user=dNvo_xcAAAAJ&hl=en", target: "_blank", Icon { width: 40, height: 40, icon: LdGraduationCap } }
                }
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
