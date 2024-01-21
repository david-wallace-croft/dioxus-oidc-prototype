use super::high_five::HighFive;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    h1 {
      "Home Page"
    }
    p {
      "This line is a placeholder for home page content."
    }
    HighFive { }
  }
}
