use super::super::route::Route;
use super::login_logout::oidc::ClientState;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  use_shared_state_provider(cx, || ClientState::default());
  render! {
    Router::<Route> { }
  }
}
