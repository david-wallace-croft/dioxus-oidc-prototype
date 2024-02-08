use super::super::route::Route;
use super::header::Header;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::log::Level;
use ::wasm_logger::Config;

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
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
