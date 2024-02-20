use self::callback_query_segments::CallbackQuerySegments;
use self::callback_state::CallbackState;
use super::login_logout::constants;
use super::login_logout::oidc::{ClientState, PkceState};
use super::login_logout::props::client::ClientProps;
use ::com_croftsoft_lib_role::Validator;
use ::dioxus::prelude::*;
use ::gloo_storage::{errors::StorageError, SessionStorage, Storage};
use ::openidconnect::core::{CoreClient, CoreTokenResponse};

pub mod callback_query_segments;
pub mod callback_state;

#[allow(non_snake_case)]
#[component]
pub fn Callback(
  cx: Scope,
  query_params: CallbackQuerySegments,
) -> Element {
  log::info!("Callback");

  let callback_state: CallbackState = update_callback_state(cx, query_params);

  // if callback_state.validate() {
  //   let nav = use_navigator(cx);
  //   nav.push(Route::Home {});
  // }

  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);

  // TODO: Is this possible?
  if use_shared_state_client_state_option.is_none() {
    return render! {
      p {
      "Initializing..."
      }
    };
  }

  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();

  to_owned![use_shared_state_client_state];

  // use_effect(
  //   cx,
  //   &use_shared_state_client_state,
  //   |use_shared_state_client_state| {
  //     return initialize(Some(use_shared_state_client_state));
  //   },
  // );

  let client_props_option: Option<ClientProps> =
    read_client_props_from_shared_state(use_shared_state_client_state);

  let pkce_verifier_option: Option<String> =
    read_or_load_pkce_verifier(&client_props_option, cx);

  let ready_to_request_token: bool = callback_state.validate()
    && validate_client_props(client_props_option.as_ref())
    && validate_pkce_verifier(pkce_verifier_option.as_ref());

  if ready_to_request_token {
    let client_props: &ClientProps = client_props_option.as_ref().unwrap();
    let oidc_client: CoreClient = client_props.client.clone();
    let authorization_code: String = callback_state.code_option.unwrap();
    let pkce_verifier: String = pkce_verifier_option.as_ref().unwrap().clone();
    // TODO: verify that state matches expected
    request_token(authorization_code, cx, oidc_client, pkce_verifier);
  }
  // TODO: use onmounted?
  render! {
  main {
    class: "app-callback",
  h1 {
  "Callback"
  }
  p {
  "query_params: {query_params:?}"
  }
  }
  }
}

fn load_pkce_verifier() -> Option<String> {
  log::info!("Loading PKCE verifier from storage...");
  let pkce_verifier_result: Result<String, StorageError> =
    SessionStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);
  match pkce_verifier_result {
    Ok(pkce_verifier) => {
      log::info!("PKCE verifier: {pkce_verifier}");
      return Some(pkce_verifier);
    },
    Err(error) => {
      log::error!("Error: {error}");
    },
  };
  None
}

// TODO: consolidate copy and paste of this function from login-logout component
fn read_client_props_from_shared_state(
  use_shared_state_client_state: UseSharedState<ClientState>
) -> Option<ClientProps> {
  log::info!("Reading client properties from shared state...");
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
  Some(client_props.clone())
}

fn read_or_load_pkce_verifier(
  client_props_option: &Option<ClientProps>,
  cx: Scope<CallbackProps>,
) -> Option<String> {
  client_props_option.as_ref()?;
  let use_shared_state_pkce_state_option: Option<&UseSharedState<PkceState>> =
    use_shared_state::<PkceState>(cx);
  // TODO: Can this ever be None?
  use_shared_state_pkce_state_option?;
  let use_shared_state_pkce_state: &UseSharedState<PkceState> =
    use_shared_state_pkce_state_option.unwrap();
  {
    let pkce_state_ref: Ref<'_, PkceState> = use_shared_state_pkce_state.read();
    let pkce_verifier_option: &Option<String> =
      &pkce_state_ref.pkce_verifier_option;
    if pkce_verifier_option.is_some() {
      return pkce_verifier_option.clone();
    }
  }
  let pkce_verifier_option: Option<String> = load_pkce_verifier();
  pkce_verifier_option.as_ref()?;
  *use_shared_state_pkce_state.write() = PkceState {
    pkce_verifier_option: pkce_verifier_option.clone(),
  };
  pkce_verifier_option
}

fn request_token(
  authorization_code: String,
  cx: Scope<CallbackProps>,
  oidc_client: CoreClient,
  pkce_verifier: String,
) {
  log::info!("Requesting token...");
  // TODO: clear the pkce verifier from session storage
  cx.spawn(async move {
    let result: Result<CoreTokenResponse, super::login_logout::errors::Error> =
      super::login_logout::oidc::token_response(
        authorization_code,
        &oidc_client,
        pkce_verifier,
      )
      .await;
    match result {
      Ok(token_response) => {
        log::info!("{token_response:#?}");
      },
      Err(error) => {
        log::error!("{error:?}");
      },
    };
  });
}

fn update_callback_state(
  cx: Scope<CallbackProps>,
  query_params: &CallbackQuerySegments,
) -> CallbackState {
  let use_shared_state_callback_state_option: Option<
    &UseSharedState<CallbackState>,
  > = use_shared_state::<CallbackState>(cx);
  // TODO: Is this possible?
  if use_shared_state_callback_state_option.is_none() {
    return CallbackState::default();
  }
  let use_shared_state_callback_state: &UseSharedState<CallbackState> =
    use_shared_state_callback_state_option.unwrap();
  let new_callback_state: CallbackState = query_params.into();
  if *use_shared_state_callback_state.read() != new_callback_state {
    *use_shared_state_callback_state.write() = new_callback_state.clone();
  }
  new_callback_state
}

fn validate_client_props(client_props_option: Option<&ClientProps>) -> bool {
  if client_props_option.is_none() {
    log::info!("Invalid client properties");
    return false;
  }
  let client_props: &ClientProps = client_props_option.unwrap();
  log::info!("Client properties: {client_props:#?}");
  true
}

fn validate_pkce_verifier(pkce_verifier_option: Option<&String>) -> bool {
  if pkce_verifier_option.is_none() {
    log::info!("Invalid PKCE verifier");
    return false;
  }
  let pkce_verifier: &String = pkce_verifier_option.unwrap();
  log::info!("PKCE verifier: {pkce_verifier:?}");
  true
}
