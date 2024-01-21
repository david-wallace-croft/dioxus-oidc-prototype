use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn PageNotFound(
  cx: Scope,
  route: Vec<String>,
) -> Element {
  render! {
    h1 {
      "Page Not Found"
    }
    pre {
      color: "red",
      "{route:?}"
    }
  }
}
