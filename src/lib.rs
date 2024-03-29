use self::components::app::App;

pub mod components;
pub mod log;
pub mod route;
pub mod storage;
pub mod window;

pub fn launch() {
  ::dioxus_web::launch(App)
}
