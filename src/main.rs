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
                p { "Assistant Professor of Computer Science" br {} "College of Basic and Applied Sciences" br {} "Middle Tennessee State University" }
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
        h1 { "Lab Openings" }
        p {
            "I am currently looking for a self-motivated graduate student to work with me as a Research Assistant."
            " Funding would cover the Fall 2026 and Spring 2027 semesters, and may be extended further."
            " You would be free to select any reasonable project of interest, so long as it is within one of my areas of expertise (see Research Interests below for the main ones)."
            " Alternatively, you are welcome to work with me on one of my existing projects (we can meet to discuss these)."
        }
        p {
            " As a note: while I do not work in \"pure\" AI/ML and hence have not listed it below, I do work on projects that combine AI with one of my other research areas."
            " For example, I am currently mentoring an undergraduate student and we are working on a system that combines LLMs with formal reasoning frameworks to automate proof construction."
        }
        p {
            " If you are interested or would like to discuss the position or projects further, feel free to "
            a { href: "mailto:devin.jean@mtsu.edu", "send me an email" }
            "!"
        }
        h1 { "Research Interests" }
        ul {
            li { "Distributed Computing and IoT" }
            li { "Embedded Programming and Robotics" }
            li { "Theoretical Computer Science" }
            li { "Automated and Computer-Assisted Proving" }
            li { "Computer Science Education and Gamification" }
        }
        h1 { "Education" }
        ul {
            li { "Ph.D. in Computer Science - Vanderbilt University" }
            li { "B.S. in Computer Science - Middle Tennessee State University" }
            li { "B.S. in Mathematics - Middle Tennessee State University" }
        }
        h1 { "Courses Taught" }
        ul {
            li { "CSCI 3160 - Intro to Assembly Language" }
            li { "CSCI 3180 - Intro to Numerical Analysis" }
            li { "CSCI 3200 - Intro to Algorithm Analysis" }
            li { "CSCI 6100 - Algorithm Analysis" }
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
