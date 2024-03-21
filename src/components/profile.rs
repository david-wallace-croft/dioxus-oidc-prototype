use super::login_logout::LoginLogout;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Profile(cx: Scope) -> Element {
  render! {
  main {
    class: "app-profile",
  h1 { "Profile" }
  LoginLogout { }
  }
  }
}
