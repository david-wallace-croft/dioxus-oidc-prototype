use std::future::Future;

use self::errors::Error;
use self::oidc::{
  authorize_url, init_oidc_client, AuthRequest, AuthRequestState,
  AuthTokenState, ClientState,
};
use self::props::client::ClientProps;
use self::query::LoginQuerySegments;
use ::dioxus::prelude::*;
use ::openidconnect::core::CoreClient;
use ::openidconnect::ClientId;

mod constants;
mod errors;
mod oidc;
mod props;
mod query;

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  let use_state_auth_request_state: &UseState<AuthRequestState> =
    use_state(cx, || AuthRequestState {
      auth_request: None,
    });
  let use_state_auth_token_state: &UseState<AuthTokenState> =
    use_state(cx, || AuthTokenState {
      id_token: None,
      refresh_token: None,
    });
  let use_state_client_state: &UseState<ClientState> =
    use_state(cx, || ClientState {
      oidc_client: None,
    });
  render! {
  div {
    class: "app-login-logout",
  // TODO: show logout button if already logged in
  button {
    onclick: move |_event| on_click_login(
      cx,
      use_state_auth_request_state,
      use_state_client_state),
    r#type: "button",
  "Login"
  }
  }
  }
}

fn on_click_login(
  cx: Scope,
  _use_state_auth_request_state: &UseState<AuthRequestState>,
  _use_state_client_state: &UseState<ClientState>,
) {
  log::info!("Clicked");
  // TODO: Why is this not called on the second click?
  use_future(cx, (), |_| async move { on_click_login_async().await });
  // let option: Option<&Result<(ClientId, CoreClient), Error>> =
  //   init_client_future.value();
}

async fn on_click_login_async() {
  log::info!("Initializing OIDC client...");
  let result: Result<(ClientId, CoreClient), Error> = init_oidc_client().await;
  if result.is_err() {
    let error: Error = result.err().unwrap();
    log::error!("{error}");
    return;
  }
  log::info!("Success");
}
