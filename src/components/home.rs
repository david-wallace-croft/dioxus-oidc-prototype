use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
  main {
    class: "app-home",
  h1 {
    "Home Page"
  }
  p {
    "This line is a placeholder for home page content."
  }
  }
  }
}
