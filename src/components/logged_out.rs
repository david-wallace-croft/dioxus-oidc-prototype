use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LoggedOut(cx: Scope) -> Element {
  render! {
  main {
    class: "app-logged-out",
  h1 { "Logged Out" }
  p { "You have been logged out." }
  }
  }
}
