use super::login_logout::constants;
use super::login_logout::oidc::ClientState;
use super::login_logout::props::client::ClientProps;
use ::dioxus::prelude::*;
use ::dioxus_router::routable::FromQuery;
use ::form_urlencoded::Parse;
use ::gloo_storage::{errors::StorageError, SessionStorage, Storage};
use ::log::Level;
use ::openidconnect::core::CoreTokenResponse;
use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::std::fmt::{self, Display, Formatter};
use ::wasm_logger::Config;

struct CallbackInitialized;

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
  let use_shared_state_callback_initialized_option: Option<
    &UseSharedState<CallbackInitialized>,
  > = use_shared_state::<CallbackInitialized>(cx);
  if use_shared_state_callback_initialized_option.is_none() {
    return render! {
      p {
        onmounted: move |_cx| {
          use_shared_state_provider(cx, || CallbackInitialized);
          let config = Config::new(Level::Debug);
          ::wasm_logger::init(config);
        },
      "Callback not yet initialized."
      }
    };
  }
  // let config = Config::new(Level::Debug);
  // ::wasm_logger::init(config);
  log::info!("Callback()");
  use_on_create(cx, || async {
    log::info!("Callback() use_on_create()");
  });
  let result: Result<String, StorageError> =
    SessionStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);
  if result.is_err() {
    let storage_error: StorageError = result.err().unwrap();
    return render! {
      p {
      "Unable to retrieve the PKCE verifier from storage:"
      }
      p {
      "{storage_error}"
      }
    };
  }
  let pkce_verifier: String = result.unwrap();
  log::info!("Callback() pkce_verifier: {pkce_verifier:?}");
  use_shared_state_provider(cx, || ClientState::default());
  let use_shared_state_client_state_option: Option<
    &UseSharedState<ClientState>,
  > = use_shared_state::<ClientState>(cx);
  let use_shared_state_client_state: &UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();
  use_effect(
    cx,
    (use_shared_state_client_state),
    |use_shared_state_client_state| async {
      log::info!("Inside effect");
    },
  );
  let client_state_ref: Ref<'_, ClientState> =
    use_shared_state_client_state.read();
  let client_props_option_ref: &Option<ClientProps> =
    &client_state_ref.oidc_client;
  let client_props_option: Option<&ClientProps> =
    client_props_option_ref.as_ref();
  if client_props_option.is_some() {
    log::info!("Client properties retrieved.");
    let client_props: &ClientProps = client_props_option.as_ref().unwrap();
    log::info!("{client_props:?}");
    if !query_params.code.is_empty() && !query_params.state.is_empty() {
      let oidc_client = client_props.client.clone();
      let authorization_code: String = query_params.code.clone();
      // TODO: verify that state matches expected
      cx.spawn(async move {
        let result: Result<
          CoreTokenResponse,
          super::login_logout::errors::Error,
        > = super::login_logout::oidc::token_response(
          authorization_code,
          &oidc_client,
          pkce_verifier,
        )
        .await;
        match result {
          Ok(token_response) => {
            log::info!("{token_response:?}");
          },
          Err(error) => {
            log::error!("{error:?}");
          },
        };
      });
    }
  }

  render! {
  main {
    class: "app-callback",
    // onmounted: move |_cx| {
    //   use_shared_state_provider(cx, || CallbackInitialized);
    //   let config = Config::new(Level::Debug);
    //   ::wasm_logger::init(config);
    // },
  h1 {
  "Callback"
  }
  p {
  "query_params: {query_params:?}"
  }
  }
  }
}
