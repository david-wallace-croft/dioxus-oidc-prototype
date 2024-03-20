use super::login_logout::client_state::ClientState;
use crate::components::login_logout::calc_has_token_response;
use crate::components::login_logout::constants::DIOXUS_FRONT_ISSUER_URL;
use crate::components::template::TokenState;
use crate::log::LogId;
use crate::route::Route;
use ::dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Navigator;

#[allow(non_snake_case)]
pub fn Profile(cx: Scope) -> Element {
  // TODO: Is this needed?
  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);

  let use_shared_state_token_state_option: Option<&UseSharedState<TokenState>> =
    use_shared_state::<TokenState>(cx);

  let has_token_response: bool =
    calc_has_token_response(use_shared_state_token_state_option);

  if !has_token_response {
    let navigator: &Navigator = use_navigator(cx);

    navigator.push(Route::LoggedOut {});
  }

  render! {
  main {
    class: "app-profile",
  h1 { "Profile" }
  button {
    onclick: move |_mouse_event: MouseEvent| on_click(
      cx,
      use_shared_state_client_state_option,
      use_shared_state_token_state_option),
    r#type: "button",
    "Logout"
  }
  }
  }
}

async fn logout(use_shared_state_token_state: UseSharedState<TokenState>) {
  *use_shared_state_token_state.write() = TokenState::default();

  // TODO: Delete other state

  // TODO: Delete user data

  revoke_token().await;
}

fn on_click(
  cx: Scope,
  // TODO: Can I revoke a token using this kind of client?
  _use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>,
  use_shared_state_token_state_option: Option<&UseSharedState<TokenState>>,
) {
  log::trace!("{} Profile.on_click()", LogId::L006);

  let has_token_response: bool =
    calc_has_token_response(use_shared_state_token_state_option);

  if has_token_response {
    let use_shared_state_token_state: &UseSharedState<TokenState> =
      use_shared_state_token_state_option.unwrap();

    to_owned![use_shared_state_token_state];

    cx.spawn(async move {
      logout(use_shared_state_token_state).await;
    });
  } else {
    todo!();
  };
}

async fn revoke_token() {
  // TODO: disable token with server
  // https://docs.aws.amazon.com/cognito/latest/developerguide/revocation-endpoint.html
  // https://docs.aws.amazon.com/cognito/latest/developerguide/token-revocation.html

  log::trace!("{} Profile.revoke_token()", LogId::L044);

  let client = reqwest::Client::new();

  let revoke_url: String = format!("{DIOXUS_FRONT_ISSUER_URL}/oauth2/revoke");

  // TODO
  let _result = client.post(revoke_url).body("").send().await;
}
