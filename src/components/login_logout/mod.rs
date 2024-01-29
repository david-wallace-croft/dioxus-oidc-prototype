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
use ::std::future::Future;

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
  let _use_state_auth_token_state: &UseState<AuthTokenState> =
    use_state(cx, || AuthTokenState {
      id_token: None,
      refresh_token: None,
    });
  let use_state_client_state: &UseState<ClientState> =
    use_state(cx, || ClientState {
      oidc_client: None,
    });
  let button_element: Element = make_button_element(cx, use_state_client_state);
  render! {
  div {
    class: "app-login-logout",
  button_element
  }
  }
}

fn make_button_element<'a>(
  cx: Scope<'a>,
  use_state_client_state: &'a UseState<ClientState>,
) -> Element<'a> {
  let client_props_option: &Option<ClientProps> =
    &use_state_client_state.oidc_client;
  if client_props_option.is_some() {
    log::info!("Client properties retrieved.");
    let client_props: &ClientProps = client_props_option.as_ref().unwrap();
    log::info!("{client_props:?}");
    return make_login_button_element(cx, false, "Login".into());
  }
  let init_client_future: &UseFuture<Result<(ClientId, CoreClient), Error>> =
    use_future(cx, (), |_| async move {
      log::info!("Initializing OIDC client...");
      init_oidc_client().await
    });
  let option: Option<&Result<(ClientId, CoreClient), Error>> =
    init_client_future.value();
  if option.is_none() {
    return make_login_button_element(cx, true, "Initializing".into());
  }
  let result: &Result<(ClientId, CoreClient), Error> = option.unwrap();
  let result_ref: Result<&(ClientId, CoreClient), &Error> = result.as_ref();
  if result.is_err() {
    let error: &Error = result_ref.unwrap_err();
    log::error!("{error}");
    return make_login_button_element(cx, true, "Error".into());
  }
  log::info!("Client properties loaded.");
  let result_value: &(ClientId, CoreClient) = result_ref.unwrap();
  let client_id: &ClientId = &result_value.0;
  let client: &CoreClient = &result_value.1;
  let client_props = ClientProps::new(client_id.clone(), client.clone());
  let client_props_option: Option<ClientProps> = Some(client_props);
  let client_state = ClientState {
    oidc_client: client_props_option,
  };
  use_state_client_state.set(client_state);
  make_login_button_element(cx, false, "Initialized".into())
}

fn make_login_button_element<'a>(
  cx: Scope<'a>,
  disabled: bool,
  label: String,
) -> Element {
  render! {
    button {
      disabled: disabled,
      onclick: move |_event| on_click_login(),
      // r#type: "button",
      "{label}"
    }
  }
}

fn on_click_login() {
  log::info!("Login clicked.");
}
