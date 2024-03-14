use self::callback_query_segments::CallbackQuerySegments;
use self::callback_state::CallbackState;
use super::login_logout::props::client::ClientProps;
use crate::components::login_logout;
use crate::components::login_logout::client_state::ClientState;
use crate::components::template::TokenState;
use crate::log::LogId;
use crate::route::Route;
use crate::storage::{self, StorageKey};
use ::com_croftsoft_lib_role::Validator;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::openidconnect::core::{CoreClient, CoreTokenResponse};

pub mod callback_query_segments;
pub mod callback_state;

#[allow(non_snake_case)]
#[component]
pub fn Callback(
  cx: Scope,
  query_params: CallbackQuerySegments,
) -> Element {
  log::trace!("{} Callback", LogId::L001);

  use_on_create(cx, || async move {
    log::trace!("{} on_create", LogId::L015);
  });

  let callback_state: CallbackState = query_params.into();

  // if callback_state.validate() {
  //   let nav = use_navigator(cx);
  //   nav.push(Route::Home {});
  // }

  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);

  let use_shared_state_token_state_option: Option<&UseSharedState<TokenState>> =
    use_shared_state::<TokenState>(cx);

  // TODO: Is this possible?
  if use_shared_state_client_state_option.is_none()
    || use_shared_state_token_state_option.is_none()
  {
    log::trace!("{} No shared state.", LogId::L041);

    return render! {
      p {
      "Initializing..."
      }
    };
  }

  render! {
  main {
    class: "app-callback",
    onmounted: move |_mounted_event: MountedEvent| {
      let use_shared_state_client_state: &UseSharedState<ClientState> =
        use_shared_state_client_state_option.unwrap();

      let use_shared_state_token_state: &UseSharedState<TokenState> =
        use_shared_state_token_state_option.unwrap();

      to_owned![use_shared_state_client_state];

      to_owned![use_shared_state_token_state];

      let callback_state: CallbackState = callback_state.clone();

      let navigator: &Navigator = use_navigator(cx);

      to_owned![navigator];

      cx.spawn(async move {
        on_mounted_async(
          callback_state,
          navigator,
          use_shared_state_client_state,
          use_shared_state_token_state).await;
      });
    },
  h1 {
  "Callback"
  }
  p {
  "query_params: {query_params:?}"
  }
  }
  }
}

async fn on_mounted_async(
  callback_state: CallbackState,
  navigator: Navigator,
  use_shared_state_client_state: UseSharedState<ClientState>,
  use_shared_state_token_state: UseSharedState<TokenState>,
) {
  log::trace!("{} on_mounted_async()", LogId::L014);

  // TODO: Move this to a shared module
  login_logout::initialize_oidc_client(&use_shared_state_client_state).await;

  let client_props_option: Option<ClientProps> =
    ClientState::read_client_props_from_shared_state(
      use_shared_state_client_state,
    );

  log::debug!(
    "{} Client properties: {client_props_option:#?}",
    LogId::L042
  );

  if client_props_option.is_some() {
    let pkce_verifier_option: Option<String> =
      storage::get(StorageKey::PkceVerifier);

    let ready_to_request_token: bool = callback_state.validate()
      && validate_client_props(client_props_option.as_ref())
      && validate_pkce_verifier(pkce_verifier_option.as_ref());

    log::debug!(
      "{} ready to request token: {ready_to_request_token}",
      LogId::L043
    );

    if ready_to_request_token {
      let client_props: &ClientProps = client_props_option.as_ref().unwrap();

      let oidc_client: CoreClient = client_props.client.clone();

      let authorization_code: String = callback_state.code_option.unwrap();

      let pkce_verifier: String =
        pkce_verifier_option.as_ref().unwrap().clone();

      // TODO: verify that state matches expected

      storage::delete(StorageKey::PkceVerifier);

      request_token(
        authorization_code,
        navigator,
        oidc_client,
        pkce_verifier,
        use_shared_state_token_state,
      )
      .await;
    }
  }
}

async fn request_token(
  authorization_code: String,
  navigator: Navigator,
  oidc_client: CoreClient,
  pkce_verifier: String,
  use_shared_state_token_state: UseSharedState<TokenState>,
) {
  log::info!("{} Requesting token...", LogId::L011);

  let result: Result<CoreTokenResponse, super::login_logout::errors::Error> =
    super::login_logout::oidc::token_response(
      authorization_code,
      &oidc_client,
      pkce_verifier,
    )
    .await;

  if let Err(error) = result {
    log::error!("{} {error:?}", LogId::L012);

    return;
  }

  let token_response: CoreTokenResponse = result.unwrap();

  // TODO: Can the token response be no?

  log::info!("{} {token_response:#?}", LogId::L013);

  *use_shared_state_token_state.write() = TokenState::new(token_response);

  let location_option: Option<String> = storage::get(StorageKey::Location);

  let Some(location) = location_option else {
    log::debug!("{} No previous location; navigating to Home", LogId::L026);

    navigator.push(Route::Home {});

    return;
  };

  storage::delete(StorageKey::Location);

  log::debug!(
    "{} Navigating to previous location: {location}",
    LogId::L027
  );

  // TODO
  // navigator.push(location);
  navigator.push(Route::Home {});
}

fn validate_client_props(client_props_option: Option<&ClientProps>) -> bool {
  if client_props_option.is_none() {
    log::info!("{} Invalid client properties", LogId::L007);
    return false;
  }
  let client_props: &ClientProps = client_props_option.unwrap();
  log::info!("{} Client properties: {client_props:#?}", LogId::L008);
  true
}

fn validate_pkce_verifier(pkce_verifier_option: Option<&String>) -> bool {
  if pkce_verifier_option.is_none() {
    log::info!("{} Invalid PKCE verifier", LogId::L009);
    return false;
  }
  let pkce_verifier: &String = pkce_verifier_option.unwrap();
  log::info!("{} PKCE verifier: {pkce_verifier:?}", LogId::L010);
  true
}
