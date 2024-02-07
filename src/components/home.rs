use ::dioxus::prelude::*;
use ::log::Level;
use ::wasm_logger::Config;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
  main {
    class: "app-home",
    onmounted: move |_cx| {
      let config = Config::new(Level::Debug);
      ::wasm_logger::init(config);
    },
  h1 {
    "Home Page"
  }
  p {
    "This line is a placeholder for home page content."
  }
  }
  }
}
