use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
  main {
    class: "app-colophon",
  h1 { "Colophon" }
  p {
  "This website was created using the Rust library ",
  a {
    href: "https://dioxuslabs.com/",
    target: "_blank",
  "Dioxus",
  },
  "."
  }
  }
  }
}
