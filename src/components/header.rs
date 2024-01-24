use super::login_logout_button::LoginLogoutButton;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
  render! {
  header {
    class: "app-header",
  span {
    class: "app-header-title",
    "CroftSoft Dioxus OIDC Prototype"
  }
  Nav { }
  div {
    class: "app-header-button",
  LoginLogoutButton { }
  }
  }
  }
}
