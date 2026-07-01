use dioxus::prelude::*;

pub fn pretty_join(items: &[Element]) -> Element {
    match items {
        [] => rsx! {},
        [first] => rsx! { {first} },
        [first, last] => rsx! { {first} " and " {last} },
        [rest @ .., last] => rsx! { for x in rest { {x} ", " } "and " {last} }
    }
}
