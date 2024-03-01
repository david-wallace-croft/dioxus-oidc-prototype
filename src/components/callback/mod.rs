use self::callback_query_segments::CallbackQuerySegments;
use self::callback_state::CallbackState;
use super::login_logout::props::client::ClientProps;
use crate::components::login_logout::client_state::ClientState;
use crate::log::LogId;
use crate::storage;
use ::com_croftsoft_lib_role::Validator;
use ::dioxus::prelude::*;
use ::openidconnect::core::{CoreClient, CoreTokenResponse};

pub mod callback_query_segments;
pub mod callback_state;

#[allow(non_snake_case)]
#[component]
pub fn Callback(
  cx: Scope,
  query_params: CallbackQuerySegments,
) -> Element {
  log::info!("{} Callback", LogId::L001);

  use_on_create(cx, || async move {
    log::info!("{} on_create", LogId::L015);
  });

  let callback_state: CallbackState = query_params.into();

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
    ClientState::read_client_props_from_shared_state(
      use_shared_state_client_state,
    );

  if client_props_option.is_some() {
    let pkce_verifier_option: Option<String> = storage::pkce_verifier_get();

    let ready_to_request_token: bool = callback_state.validate()
      && validate_client_props(client_props_option.as_ref())
      && validate_pkce_verifier(pkce_verifier_option.as_ref());

    if ready_to_request_token {
      let client_props: &ClientProps = client_props_option.as_ref().unwrap();
      let oidc_client: CoreClient = client_props.client.clone();
      let authorization_code: String = callback_state.code_option.unwrap();
      let pkce_verifier: String =
        pkce_verifier_option.as_ref().unwrap().clone();
      // TODO: verify that state matches expected
      storage::pkce_verifier_delete();
      request_token(authorization_code, cx, oidc_client, pkce_verifier);

      let location_option: Option<String> = storage::location_get();

      if let Some(location) = location_option {
        log::info!("{} Previous location: {location}", LogId::L026);
      }
    }
  }
  render! {
  main {
    class: "app-callback",
    onmounted: move |_event| {
      cx.spawn(async move {
        log::info!("{} onmounted", LogId::L014);
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

fn request_token(
  authorization_code: String,
  cx: Scope<CallbackProps>,
  oidc_client: CoreClient,
  pkce_verifier: String,
) {
  log::info!("{} Requesting token...", LogId::L011);
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
        log::info!("{} {token_response:#?}", LogId::L012);
      },
      Err(error) => {
        log::error!("{} {error:?}", LogId::L013);
      },
    };
  });
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
