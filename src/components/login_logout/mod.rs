use self::errors::Error;
use self::oidc::AuthRequest;
use self::props::client::ClientProps;
use crate::components::login_logout::client_state::ClientState;
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

#[derive(PartialEq)]
enum ButtonState {
  Loading,
  Login,
  Logout,
}

#[allow(non_snake_case)]
pub fn LoginLogout(cx: Scope) -> Element {
  let use_state_button_state: &UseState<ButtonState> =
    use_state(cx, || ButtonState::Login);

  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);

  let button_label: &str = match use_state_button_state.get() {
    ButtonState::Loading => "...",
    ButtonState::Login => "Login",
    ButtonState::Logout => "Logout",
  };

  render! {
  div {
    class: "app-login-logout",
    onmounted: move |_mounted_event: MountedEvent| on_mounted(
      use_state_button_state),
  button {
    onclick: move |_mouse_event: MouseEvent| on_click(
      cx,
      use_shared_state_client_state_option,
      use_state_button_state),
    r#type: "button",
    "{button_label}"
  }
  }
  }
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
  use_state_button_state: &UseState<ButtonState>,
) {
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();

  to_owned![use_shared_state_client_state];

  to_owned![use_state_button_state];

  cx.spawn(async move {
    login_async(use_shared_state_client_state, use_state_button_state).await
  });
}

async fn login_async(
  use_shared_state_client_state: UseSharedState<ClientState>,
  use_state_button_state: UseState<ButtonState>,
) {
  use_state_button_state.set(ButtonState::Loading);

  initialize_oidc_client(&use_shared_state_client_state).await;

  let client_props_option: Option<ClientProps> =
    ClientState::read_client_props_from_shared_state(
      use_shared_state_client_state.clone(),
    );

  if client_props_option.is_none() {
    log::trace!("{} No client properties.", LogId::L028);

    use_state_button_state.set(ButtonState::Login);

    return;
  }

  let Some(location) = window::get_location() else {
    log::trace!("{} No window location.", LogId::L029);

    use_state_button_state.set(ButtonState::Login);

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
    use_state_button_state.set(ButtonState::Login);

    return;
  }

  let window: Window = window_option.unwrap();

  let _result = window.open_with_url_and_target(authorize_url_str, "_self");
}

fn logout(use_state_button_state: &UseState<ButtonState>) {
  // TODO: disable token with server
  // https://docs.aws.amazon.com/cognito/latest/developerguide/revocation-endpoint.html
  // https://docs.aws.amazon.com/cognito/latest/developerguide/token-revocation.html

  storage::delete(StorageKey::TokenResponse);

  // TODO: Delete user data

  use_state_button_state.set(ButtonState::Login);
}

fn on_click(
  cx: Scope,
  use_shared_state_client_state_option: Option<&UseSharedState<ClientState>>,
  use_state_button_state: &UseState<ButtonState>,
) {
  log::trace!("{} LoginLogout.on_click()", LogId::L006);

  match *use_state_button_state.get() {
    ButtonState::Loading => {
      // TODO: Maybe try to load the client again here
    },
    ButtonState::Login => login(
      cx,
      use_shared_state_client_state_option,
      use_state_button_state,
    ),
    ButtonState::Logout => logout(use_state_button_state),
  };
}

fn on_mounted(use_state_button_state: &UseState<ButtonState>) {
  log::trace!("{} LoginLogout.on_mounted()", LogId::L032);

  let token_response_option: Option<CoreTokenResponse> =
    storage::get(StorageKey::TokenResponse);

  if token_response_option.is_some() {
    // TODO: Check that the token has not expired

    // TODO: Schedule a token refresh

    use_state_button_state.set(ButtonState::Logout);

    return;
  }
}
