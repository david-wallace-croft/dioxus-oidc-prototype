use super::login_logout::constants;
use super::login_logout::oidc::{ClientState, PkceState};
use super::login_logout::props::client::ClientProps;
use ::dioxus::prelude::*;
use ::dioxus_router::routable::FromQuery;
use ::form_urlencoded::Parse;
use ::gloo_storage::{errors::StorageError, SessionStorage, Storage};
use ::openidconnect::core::CoreTokenResponse;
use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::std::fmt::{self, Display, Formatter};
use openidconnect::core::CoreClient;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CallbackQuerySegments {
  pub code: String,
  pub state: String,
}

impl Display for CallbackQuerySegments {
  fn fmt(
    &self,
    formatter: &mut Formatter<'_>,
  ) -> fmt::Result {
    write!(formatter, "code={}&state={}", self.code, self.state)
  }
}

impl FromQuery for CallbackQuerySegments {
  fn from_query(query: &str) -> Self {
    let mut code_option: Option<String> = None;
    let mut state_option: Option<String> = None;
    let input: &[u8] = query.as_bytes();
    let pairs: Parse<'_> = ::form_urlencoded::parse(input);
    pairs.for_each(|(key, value): (Cow<'_, str>, Cow<'_, str>)| {
      if key == "code" {
        code_option = Some(value.clone().into());
      } else if key == "state" {
        state_option = Some(value.clone().into());
      }
    });
    Self {
      code: code_option.unwrap_or_default(),
      state: state_option.unwrap_or_default(),
    }
  }
}

#[allow(non_snake_case)]
#[component]
pub fn Callback(
  cx: Scope,
  query_params: CallbackQuerySegments,
) -> Element {
  log::info!("Callback");
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
  let mut ready_to_request_token: bool = true;
  if query_params.code.is_empty() {
    log::info!("query_params.code is empty");
    ready_to_request_token = false;
  } else {
    let query_params_code = &query_params.code;
    log::info!("query_params.code: {query_params_code}");
  }
  if query_params.state.is_empty() {
    log::info!("query_params.state is empty");
    ready_to_request_token = false;
  } else {
    let query_params_state = &query_params.state;
    log::info!("query_params.state: {query_params_state}");
  }
  if client_props_option.is_none() {
    log::info!("client_props_option is None");
    ready_to_request_token = false;
  } else {
    let client_props: &ClientProps = client_props_option.as_ref().unwrap();
    log::info!("client_props: {client_props:#?}");
  }
  if pkce_verifier_option.is_none() {
    log::info!("pkce_verifier_option is None");
    ready_to_request_token = false;
  } else {
    let pkce_verifier: String = pkce_verifier_option.as_ref().unwrap().clone();
    log::info!("pkce_verifier: {pkce_verifier:?}");
  }
  if ready_to_request_token {
    let client_props: &ClientProps = client_props_option.as_ref().unwrap();
    let oidc_client: CoreClient = client_props.client.clone();
    let authorization_code: String = query_params.code.clone();
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
  // p {
  // "pkce_verifier_option: {pkce_verifier_option:?}"
  // }
  // p {
  // "client_props_option: {client_props_option:?}"
  // }
  }
  }
}

fn read_or_load_pkce_verifier(
  client_props_option: &Option<ClientProps>,
  cx: Scope<CallbackProps>,
) -> Option<String> {
  if client_props_option.is_none() {
    return None;
  }
  let use_shared_state_pkce_state_option: Option<&UseSharedState<PkceState>> =
    use_shared_state::<PkceState>(cx);
  if use_shared_state_pkce_state_option.is_none() {
    // TODO: Can this happen?
    return None;
  }
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
  let pkce_verifier_option = load_pkce_verifier();
  if pkce_verifier_option.is_none() {
    return None;
  }
  *use_shared_state_pkce_state.write() = PkceState {
    pkce_verifier_option: pkce_verifier_option.clone(),
  };
  pkce_verifier_option
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

fn request_token(
  authorization_code: String,
  cx: Scope<CallbackProps>,
  oidc_client: CoreClient,
  pkce_verifier: String,
) {
  log::info!("Requesting token...");
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
