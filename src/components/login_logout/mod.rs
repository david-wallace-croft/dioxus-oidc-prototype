use self::errors::Error;
use self::oidc::{
  authorize_url, init_oidc_client, AuthRequest, AuthRequestState,
  AuthTokenState, ClientState,
};
use self::props::client::ClientProps;
use ::dioxus::prelude::*;
use ::openidconnect::core::CoreClient;
use ::openidconnect::ClientId;
use ::web_sys::{window, Window};

mod constants;
pub mod errors;
pub mod oidc;
pub mod props;

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
  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();
  use_on_create(cx, || {
    to_owned![use_state_auth_request_state];
    to_owned![use_shared_state_client_state];
    initialize_oidc_client(
      use_state_auth_request_state,
      use_shared_state_client_state,
    )
  });
  render! {
  div {
    class: "app-login-logout",
    button {
      // disabled: use_state_auth_request_state.auth_request.is_none(),
      onclick: move |_event| on_click_login(use_state_auth_request_state.clone()),
      r#type: "button",
      "Login"
    }
  }
  }
}

async fn initialize_oidc_client(
  use_state_auth_request_state: UseState<AuthRequestState>,
  use_shared_state_client_state: UseSharedState<ClientState>,
) {
  {
    let client_state_ref: Ref<'_, ClientState> =
      use_shared_state_client_state.read();
    let client_props_option_ref: &Option<ClientProps> =
      &client_state_ref.oidc_client;
    let client_props_option: Option<&ClientProps> =
      client_props_option_ref.as_ref();
    if client_props_option.is_some() {
      log::info!("Client properties loaded from shared state.");
      let client_props: &ClientProps = client_props_option.as_ref().unwrap();
      log::info!("{client_props:?}");
      return;
    }
  }
  log::info!("Initializing OIDC client...");
  let result: Result<(ClientId, CoreClient), Error> = init_oidc_client().await;
  if result.is_err() {
    let error: Error = result.unwrap_err();
    log::error!("{error}");
    return;
  }
  log::info!("Client properties initialized.");
  let result_value = result.unwrap();
  let client_id: ClientId = result_value.0;
  let client: CoreClient = result_value.1;
  let client_props = ClientProps::new(client_id.clone(), client.clone());
  let client_props_option: Option<ClientProps> = Some(client_props);
  let client_state = ClientState {
    oidc_client: client_props_option,
  };
  let mut client_state_ref_mut: RefMut<'_, ClientState> =
    use_shared_state_client_state.write();
  *client_state_ref_mut = client_state;
  log::info!("Client properties saved to shared state.");
  let auth_request: AuthRequest = authorize_url(client.clone());
  let auth_request_state = AuthRequestState {
    auth_request: Some(auth_request.clone()),
  };
  use_state_auth_request_state.set(auth_request_state);
}

fn on_click_login(use_state_auth_request_state: UseState<AuthRequestState>) {
  log::info!("Login clicked.");
  let auth_request_ref_option: Option<&AuthRequest> =
    use_state_auth_request_state.auth_request.as_ref();
  if auth_request_ref_option.is_none() {
    return;
  }
  let auth_request_ref: &AuthRequest = auth_request_ref_option.unwrap();
  let authorize_url_str: &str = auth_request_ref.authorize_url.as_str();
  log::info!("URL: {authorize_url_str}");
  let window_option: Option<Window> = window();
  if window_option.is_none() {
    return;
  }
  let window: Window = window_option.unwrap();
  let _result = window.open_with_url_and_target(authorize_url_str, "_self");
}
