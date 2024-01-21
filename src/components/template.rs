use crate::components::nav::Nav;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
  render! {
    Nav { }
    Outlet::<Route> {}
  }
}
