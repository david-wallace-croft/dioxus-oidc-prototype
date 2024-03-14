use super::super::route::Route;
use super::header::Header;
use super::login_logout::client_state::ClientState;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::log::Level;
use ::openidconnect::core::CoreTokenResponse;
use ::wasm_logger::Config;

#[derive(Default)]
pub struct TokenState {
  pub core_token_response_option: Option<CoreTokenResponse>,
}

impl TokenState {
  pub fn new(core_token_response: CoreTokenResponse) -> Self {
    Self {
      core_token_response_option: Some(core_token_response),
    }
  }
}

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
  use_shared_state_provider(cx, ClientState::default);

  use_shared_state_provider(cx, TokenState::default);

  use_on_create(cx, || async {
    let config = Config::new(Level::Trace);
    ::wasm_logger::init(config);
  });

  render! {
  div {
    class: "app-template",
  Header { }
  Outlet::<Route> {}
  }
  }
}
