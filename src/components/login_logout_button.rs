use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LoginLogoutButton(cx: Scope) -> Element {
  render! {
    button {
      class: "app-login-logout-button",
      r#type: "button",
      "Login / Logout Button"
    }
  }
}
