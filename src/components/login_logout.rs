use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  render! {
  div {
    class: "app-login-logout",
  button {
    class: "app-login-logout",
    r#type: "button",
  "Login / Logout Button"
  }
  }
  }
}
