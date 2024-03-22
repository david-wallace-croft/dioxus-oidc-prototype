use super::login_logout::LoginLogout;
use crate::components::login_logout::extract_token_response;
use crate::components::template::TokenState;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Profile(cx: Scope) -> Element {
  let use_shared_state_token_state_option: Option<&UseSharedState<TokenState>> =
    use_shared_state::<TokenState>(cx);

  let core_token_response_option =
    extract_token_response(use_shared_state_token_state_option);

  render! {
  main {
    class: "app-profile",
  h1 { "Profile" }
  LoginLogout { }
  if let Some(core_token_response) = core_token_response_option {
    let token_string = format!("{:#?}", core_token_response);

    render! {
    h2 { "Token" }
    div {
      style: "text-align: left; white-space: pre-wrap;",
    "{token_string}"
    }
    }
  }
  }
  }
}
