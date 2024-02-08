use super::login_logout::constants;
use super::login_logout::oidc::ClientState;
use super::login_logout::props::client::ClientProps;
use ::dioxus::prelude::*;
use ::dioxus_router::routable::FromQuery;
use ::form_urlencoded::Parse;
use ::gloo_storage::{errors::StorageError, SessionStorage, Storage};
use ::openidconnect::core::CoreTokenResponse;
use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum CallbackPhase {
  INITIALIZED,
  UNINITIALIZED,
}

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
  // TODO: Can this be use_state instead of shared state?
  use_shared_state_provider(cx, || CallbackPhase::UNINITIALIZED);
  let use_shared_state_callback_phase_option: Option<
    &UseSharedState<CallbackPhase>,
  > = use_shared_state::<CallbackPhase>(cx);
  let use_shared_state_callback_phase_ref: &UseSharedState<CallbackPhase> =
    use_shared_state_callback_phase_option.unwrap();
  let callback_phase_ref: Ref<CallbackPhase> =
    use_shared_state_callback_phase_ref.read();
  let callback_phase: &CallbackPhase = &callback_phase_ref;
  // use_effect(
  //   cx,
  //   use_shared_state_callback_phase,
  //   |use_shared_state_callback_phase| async {
  //     // let callback_phase_ref: Ref<CallbackPhase> =
  //     //   use_shared_state_callback_phase.read();
  //     // let callback_phase: CallbackPhase = *callback_phase_ref;
  //     // log::info!("Inside effect: {callback_phase:?}");
  //   },
  // );

  // use_shared_state_provider(cx, || ClientState::default());
  // let use_shared_state_client_state_option: Option<
  //   &UseSharedState<ClientState>,
  // > = use_shared_state::<ClientState>(cx);
  // if use_shared_state_client_state_option.is_none() {
  //   return render! {
  //     p {
  //     "Initializing..."
  //     }
  //   };
  // }
  // let use_shared_state_client_state: &UseSharedState<ClientState> =
  //   use_shared_state_client_state_option.unwrap();
  // to_owned![use_shared_state_client_state];
  // use_effect(
  //   cx,
  //   &use_shared_state_client_state,
  //   |use_shared_state_client_state| {
  //     return initialize(Some(use_shared_state_client_state));
  //   },
  // );
  // to_owned![use_shared_state_client_state];
  // let client_props_option: Option<ClientProps> =
  //   read_client_props_from_shared_state(use_shared_state_client_state);

  // use_on_create(cx, || {
  //   if use_shared_state_client_state_option.is_none() {
  //     return initialize(None);
  //   }
  //   let use_shared_state_client_state: &UseSharedState<ClientState> =
  //     use_shared_state_client_state_option.unwrap();
  //   to_owned![use_shared_state_client_state];
  //   return initialize(Some(use_shared_state_client_state));

  //   // let config = Config::new(Level::Debug);
  //   // ::wasm_logger::init(config);
  //   // log::info!("Inside effect");
  //   // to_owned![use_shared_state_client_state];
  //   // async {
  //   //   let client_state_ref: Ref<'_, ClientState> =
  //   //     use_shared_state_client_state.read();
  //   //   let client_props_option_ref: &Option<ClientProps> =
  //   //     &client_state_ref.oidc_client;
  //   //   let client_props_option: Option<&ClientProps> =
  //   //     client_props_option_ref.as_ref();
  //   // }
  // });

  // let client_state_ref: Ref<'_, ClientState> =
  //   use_shared_state_client_state.read();
  // let client_props_option_ref: &Option<ClientProps> =
  //   &client_state_ref.oidc_client;
  // let client_props_option: Option<&ClientProps> =
  //   client_props_option_ref.as_ref();
  // if client_props_option.is_none() {
  //   return render! {
  //     p {
  //     "Callback not yet initialized."
  //     }
  //   };
  // }
  // let use_shared_state_callback_initialized_option: Option<
  //   &UseSharedState<CallbackInitialized>,
  // > = use_shared_state::<CallbackInitialized>(cx);
  // if use_shared_state_callback_initialized_option.is_none() {
  //   return render! {
  //     p {
  //       onmounted: move |_cx| {
  //         use_shared_state_provider(cx, || CallbackInitialized);
  //         let config = Config::new(Level::Debug);
  //         ::wasm_logger::init(config);
  //       },
  //     "Callback not yet initialized."
  //     }
  //   };
  // }
  // let config = Config::new(Level::Debug);
  // ::wasm_logger::init(config);
  // log::info!("Callback()");
  // use_on_create(cx, || async {
  //   log::info!("Callback() use_on_create()");
  // });
  // let result: Result<String, StorageError> =
  //   SessionStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);
  // if result.is_err() {
  //   let storage_error: StorageError = result.err().unwrap();
  //   return render! {
  //     p {
  //     "Unable to retrieve the PKCE verifier from storage:"
  //     }
  //     p {
  //     "{storage_error}"
  //     }
  //   };
  // }
  // let pkce_verifier: String = result.unwrap();
  // log::info!("Callback() pkce_verifier: {pkce_verifier:?}");
  // use_shared_state_provider(cx, || ClientState::default());
  // let use_shared_state_client_state_option: Option<
  //   &UseSharedState<ClientState>,
  // > = use_shared_state::<ClientState>(cx);
  // let use_shared_state_client_state: &UseSharedState<ClientState> =
  //   use_shared_state_client_state_option.unwrap();
  // use_effect(
  //   cx,
  //   (use_shared_state_client_state),
  //   |use_shared_state_client_state| async {
  //     log::info!("Inside effect");
  //   },
  // );
  // let client_state_ref: Ref<'_, ClientState> =
  //   use_shared_state_client_state.read();
  // let client_props_option_ref: &Option<ClientProps> =
  //   &client_state_ref.oidc_client;
  // let client_props_option: Option<&ClientProps> =
  //   client_props_option_ref.as_ref();
  // if client_props_option.is_some() {
  // log::info!("Client properties retrieved.");
  // let client_props: &ClientProps = client_props_option.as_ref().unwrap();
  // log::info!("{client_props:?}");
  // if !query_params.code.is_empty() && !query_params.state.is_empty() {
  //   let oidc_client = client_props.client.clone();
  //   let authorization_code: String = query_params.code.clone();
  //   // TODO: verify that state matches expected
  //   cx.spawn(async move {
  //     let result: Result<
  //       CoreTokenResponse,
  //       super::login_logout::errors::Error,
  //     > = super::login_logout::oidc::token_response(
  //       authorization_code,
  //       &oidc_client,
  //       pkce_verifier,
  //     )
  //     .await;
  //     match result {
  //       Ok(token_response) => {
  //         log::info!("{token_response:?}");
  //       },
  //       Err(error) => {
  //         log::error!("{error:?}");
  //       },
  //     };
  //   });
  //   // }
  // }

  render! {
  main {
    class: "app-callback",
    onmounted: move |_event| on_mounted(cx),
  h1 {
  "Callback"
  }
  p {
  "query_params: {query_params:?}"
  }
  p {
  "callback_phase: {callback_phase:?}"
  }
  }
  }
}

async fn initialize(
  use_shared_state_client_state_option: Option<UseSharedState<ClientState>>
) {
  log::info!("Callback initialize()");
  if use_shared_state_client_state_option.is_none() {
    return;
  }
  let use_shared_state_client_state: UseSharedState<ClientState> =
    use_shared_state_client_state_option.unwrap();
  let client_props_option: Option<ClientProps> =
    read_client_props_from_shared_state(use_shared_state_client_state);
  if client_props_option.is_none() {
    return;
  }
  log::info!("{client_props_option:?}");
}

fn on_mounted(cx: Scope<CallbackProps>) {
  log::info!("Logging initialized in Callback onmounted.");
  let use_shared_state_callback_phase_option: Option<
    &UseSharedState<CallbackPhase>,
  > = use_shared_state::<CallbackPhase>(cx);
  let use_shared_state_callback_phase: &UseSharedState<CallbackPhase> =
    use_shared_state_callback_phase_option.unwrap();
  *use_shared_state_callback_phase.write() = CallbackPhase::INITIALIZED;
}

// TODO: consolidate copy and paste of this function from login-logout component
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
  log::info!("{client_props:?}");
  Some(client_props.clone())
}
