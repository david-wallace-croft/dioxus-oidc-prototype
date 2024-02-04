use self::components::app::App;
// use ::log::Level;
// use ::wasm_logger::Config;

pub mod components;
pub mod route;

pub fn launch() {
  // let config = Config::new(Level::Debug);
  // ::wasm_logger::init(config);
  ::dioxus_web::launch(App)
}
