use self::client_state::ClientState;
use self::constants::DIOXUS_FRONT_ISSUER_URL;
use self::errors::Error;
use self::oidc::AuthRequest;
use self::props::client::ClientProps;
use crate::components::template::TokenState;
use crate::log::LogId;
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
  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);

  let use_shared_state_token_state_option: Option<&UseSharedState<TokenState>> =
    use_shared_state::<TokenState>(cx);

  let has_token_response: bool =
    calc_has_token_response(use_shared_state_token_state_option);

  let button_label: &str = if has_token_response {
    "Logout"
  } else {
    "Login"
  };

  render! {
  div {
    class: "app-login-logout",
    onmounted: move |_mounted_event: MountedEvent| on_mounted(),
  button {
    onclick: move |_mouse_event: MouseEvent| on_click(
      cx,
      use_shared_state_client_state_option,
      use_shared_state_token_state_option),
    r#type: "button",
    "{button_label}"
  }
  }
  }
}

fn calc_has_token_response(
  use_shared_state_token_state_option: Option<&UseSharedState<TokenState>>
) -> bool {
  if use_shared_state_token_state_option.is_none() {
    log::trace!("{} No token state.", LogId::L037);

    return false;
  }

  let use_shared_state_token_state: &UseSharedState<TokenState> =
    use_shared_state_token_state_option.unwrap();

  let token_state_ref: Ref<'_, TokenState> =
    use_shared_state_token_state.read();

  let core_token_response_option_ref: &Option<CoreTokenResponse> =
    &token_state_ref.core_token_response_option;

  if core_token_response_option_ref.is_none() {
    log::trace!("{} No token response.", LogId::L040);

    return false;
  }

  let core_token_response_ref_option: Option<&CoreTokenResponse> =
    core_token_response_option_ref.as_ref();

  let core_token_response_ref: &CoreTokenResponse =
    core_token_response_ref_option.unwrap();

  log::debug!(
    "{} Token response: {core_token_response_ref:#?}",
    LogId::L045
  );

  // TODO: Check that the token has not expired

  // TODO: Schedule a token refresh

  true
}

pub async fn initialize_oidc_client(
  use_shared_state_client_state: &UseSharedState<ClientState>
) {
  log::trace!("{} LoginLogout.initialize_oidc_client()", LogId::L038);

  // TODO: Is this still needed?
  if ClientState::read_client_props_from_shared_state(
    use_shared_state_client_state.clone(),
  )
  .is_some()
  {
    return;
  }

  log::trace!("{} Initializing OIDC client...", LogId::L023);

  let result: Result<(ClientId, CoreClient), Error> =
    oidc::init_oidc_client().await;

  if result.is_err() {
    let error: Error = result.unwrap_err();

    log::error!("{error}");

    return;
  }

  log::trace!("{} Client properties initialized.", LogId::L024);

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

  log::trace!("{} Client properties saved to shared state.", LogId::L025);
}

fn login(
  cx: Scope,
  use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>,
) {
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();

  to_owned![use_shared_state_client_state];

  cx.spawn(async move { login_async(use_shared_state_client_state).await });
}

async fn login_async(
  use_shared_state_client_state: UseSharedState<ClientState>
) {
  // TODO: disable the button

  initialize_oidc_client(&use_shared_state_client_state).await;

  let client_props_option: Option<ClientProps> =
    ClientState::read_client_props_from_shared_state(
      use_shared_state_client_state.clone(),
    );

  if client_props_option.is_none() {
    log::trace!("{} No client properties.", LogId::L028);

    // TODO: re-enable the button

    return;
  }

  let Some(location) = window::get_location() else {
    log::trace!("{} No window location.", LogId::L029);

    // TODO: re-enable the button

    return;
  };

  log::debug!("{} login() Location: {location}", LogId::L016);

  // TODO: What if the result is Err?
  let _result = storage::set(StorageKey::Location, &location);

  let client_props: ClientProps = client_props_option.unwrap();

  let client: CoreClient = client_props.client;

  let auth_request: AuthRequest = oidc::authorize_url(client);

  let authorize_url_str: &str = auth_request.authorize_url.as_str();

  log::debug!("{} login() Authorize URL: {authorize_url_str}", LogId::L017);

  let window_option: Option<Window> = window();

  if window_option.is_none() {
    // TODO: re-enable the button

    return;
  }

  let window: Window = window_option.unwrap();

  let _result = window.open_with_url_and_target(authorize_url_str, "_self");
}

async fn logout(use_shared_state_token_state: UseSharedState<TokenState>) {
  *use_shared_state_token_state.write() = TokenState::default();

  // TODO: Delete other state

  // TODO: Delete user data

  revoke_token().await;
}

fn on_click(
  cx: Scope,
  use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>,
  use_shared_state_token_state_option: Option<&UseSharedState<TokenState>>,
) {
  log::trace!("{} LoginLogout.on_click()", LogId::L006);

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
    login(cx, use_shared_state_client_state_option);
  };
}

fn on_mounted() {
  log::trace!("{} LoginLogout.on_mounted()", LogId::L032);
}

async fn revoke_token() {
  // TODO: disable token with server
  // https://docs.aws.amazon.com/cognito/latest/developerguide/revocation-endpoint.html
  // https://docs.aws.amazon.com/cognito/latest/developerguide/token-revocation.html

  log::trace!("{} LoginLogout.revoke_token()", LogId::L044);

  let client = reqwest::Client::new();

  let revoke_url: String = format!("{DIOXUS_FRONT_ISSUER_URL}/oauth2/revoke");

  let _result = client.post(revoke_url).body("").send().await;

  // let token_response_option: Option<CoreTokenResponse> =
  //   storage::get(StorageKey::TokenResponse);

  // if token_response_option.is_none() {
  //   log::trace!("{} No token response.", LogId:XXXXX);

  //   return;
  // }

  // let token_response: CoreTokenResponse = token_response_option.unwrap();

  // let client_props_option: Option<ClientProps> =
  //   ClientState::read_client_props_from_shared_state(
  //     use_shared_state::<ClientState>(cx),
  //   );

  // if client_props_option.is_none() {
  //   log::trace!("{} No client properties.", LogId:XXXXX);

  //   return;
  // }

  // let client_props: ClientProps = client_props_option.unwrap();

  // let client: CoreClient = client_props.client;

  // let revoke_token_request: oidc::RevokeTokenRequest =
  //   oidc::revoke_token_request(client, token_response);
}
