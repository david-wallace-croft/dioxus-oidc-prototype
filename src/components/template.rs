use super::super::route::Route;
use super::header::Header;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
  render! {
    Header { }
    Outlet::<Route> {}
  }
}
