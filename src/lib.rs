use self::components::app::App;

pub mod components;
pub mod route;

pub fn launch() {
  ::dioxus_web::launch(App)
}
