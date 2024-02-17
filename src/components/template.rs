use super::super::route::Route;
use super::callback::CallbackState;
use super::header::Header;
use super::login_logout::oidc::{ClientState, PkceState};
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::log::Level;
use ::wasm_logger::Config;

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
  use_shared_state_provider(cx, CallbackState::default);
  use_shared_state_provider(cx, ClientState::default);
  use_shared_state_provider(cx, PkceState::default);
  use_on_create(cx, || async {
    let config = Config::new(Level::Debug);
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
