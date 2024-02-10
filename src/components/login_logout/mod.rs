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

pub mod constants;
pub mod errors;
pub mod oidc;
pub mod props;

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  let use_state_label: &UseState<i32> = use_state(cx, || 0);
  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);
  render! {
  div {
    class: "app-login-logout",
    onmounted: move |_event| {
      let use_shared_state_client_state: &UseSharedState<ClientState> =
        use_shared_state_client_state_option.unwrap();
      to_owned![use_shared_state_client_state];
      cx.spawn(async move {
        initialize_oidc_client(use_shared_state_client_state).await
      });
    },
  button {
    onclick: move |_event| on_click_login(use_shared_state_client_state_option, use_state_label),
    r#type: "button",
    "Login {use_state_label}"
  }
  }
  }
}

async fn initialize_oidc_client(
  use_shared_state_client_state: UseSharedState<ClientState>
) {
  // TODO: Is this still needed?
  if read_client_props_from_shared_state(use_shared_state_client_state.clone())
    .is_some()
  {
    return;
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
}

fn on_click_login(
  use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>,
  use_state_label: &UseState<i32>,
) {
  log::info!("Login clicked.");
  use_state_label.set(1);
  if use_shared_state_client_state_option.is_none() {
    use_state_label.set(2);
    return;
  }
  use_state_label.set(3);
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();
  let client_props_option: Option<ClientProps> =
    read_client_props_from_shared_state(use_shared_state_client_state.clone());
  if client_props_option.is_none() {
    use_state_label.set(4);
    return;
  }
  use_state_label.set(5);
  let client_props: ClientProps = client_props_option.unwrap();
  let client: CoreClient = client_props.client;
  let auth_request: AuthRequest = authorize_url(client);
  let authorize_url_str: &str = auth_request.authorize_url.as_str();
  log::info!("on_click_login() Authorize URL: {authorize_url_str}");
  let window_option: Option<Window> = window();
  if window_option.is_none() {
    use_state_label.set(6);
    return;
  }
  use_state_label.set(7);
  let window: Window = window_option.unwrap();
  let _result = window.open_with_url_and_target(authorize_url_str, "_self");
}

fn read_client_props_from_shared_state(
  use_shared_state_client_state: UseSharedState<ClientState>
) -> Option<ClientProps> {
  let client_state_ref: Ref<'_, ClientState> =
    use_shared_state_client_state.read();
  let client_props_option_ref: &Option<ClientProps> =
    &client_state_ref.oidc_client;
  if client_props_option_ref.is_none() {
    return None;
  }
  let client_props_option: Option<&ClientProps> =
    client_props_option_ref.as_ref();
  log::info!("Client properties loaded from shared state.");
  let client_props: &ClientProps = client_props_option.unwrap();
  log::info!("{client_props:#?}");
  Some(client_props.clone())
}
