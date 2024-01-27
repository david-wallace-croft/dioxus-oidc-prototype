use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  render! {
  div {
    class: "app-login-logout",
  // TODO: show logout button if already logged in
  button {
    onclick: move |_event| on_click(cx),
    r#type: "button",
  "Login"
  }
  }
  }
}

fn on_click(cx: Scope) {
  log::info!("Clicked");
}
