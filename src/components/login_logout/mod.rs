use self::errors::Error;
use self::oidc::AuthRequest;
use self::props::client::ClientProps;
use crate::components::login_logout::client_state::ClientState;
use crate::storage::StorageKey;
use crate::{storage, window};
use ::dioxus::prelude::*;
use ::openidconnect::core::CoreClient;
use ::openidconnect::core::CoreTokenResponse;
use ::openidconnect::ClientId;
use ::web_sys::{window, Window};

pub mod client_state;
pub mod constants;
pub mod errors;
pub mod oidc;
pub mod props;

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  let use_state_label: &UseState<String> = use_state(cx, || "...".into());

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

      to_owned![use_state_label];

      cx.spawn(async move {
        initialize(use_shared_state_client_state, use_state_label).await
      });
    },
  button {
    onclick: move |_event| on_click_login(use_shared_state_client_state_option),
    r#type: "button",
    "{use_state_label}"
  }
  }
  }
}

async fn initialize(
  use_shared_state_client_state: UseSharedState<ClientState>,
  use_state_label: UseState<String>,
) {
  let token_response_option: Option<CoreTokenResponse> =
    storage::get(StorageKey::TokenResponse);

  if token_response_option.is_some() {
    use_state_label.set("Logout".into());

    return;
  }

  initialize_oidc_client(use_shared_state_client_state, use_state_label).await;
}

async fn initialize_oidc_client(
  use_shared_state_client_state: UseSharedState<ClientState>,
  use_state_label: UseState<String>,
) {
  // TODO: Is this still needed?
  if ClientState::read_client_props_from_shared_state(
    use_shared_state_client_state.clone(),
  )
  .is_some()
  {
    return;
  }

  log::info!("Initializing OIDC client...");

  let result: Result<(ClientId, CoreClient), Error> =
    oidc::init_oidc_client().await;

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
  use_state_label.set("Login".into());
}

fn on_click_login(
  use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>
) {
  log::info!("Login clicked.");
  if use_shared_state_client_state_option.is_none() {
    return;
  }
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();
  let client_props_option: Option<ClientProps> =
    ClientState::read_client_props_from_shared_state(
      use_shared_state_client_state.clone(),
    );
  if client_props_option.is_none() {
    return;
  }
  let Some(location) = window::get_location() else {
    return;
  };
  log::info!("on_click_login() Location: {location}");
  storage::location_set(&location);
  let client_props: ClientProps = client_props_option.unwrap();
  let client: CoreClient = client_props.client;
  let auth_request: AuthRequest = oidc::authorize_url(client);
  let authorize_url_str: &str = auth_request.authorize_url.as_str();
  log::info!("on_click_login() Authorize URL: {authorize_url_str}");
  let window_option: Option<Window> = window();
  if window_option.is_none() {
    return;
  }
  let window: Window = window_option.unwrap();
  let _result = window.open_with_url_and_target(authorize_url_str, "_self");
}
