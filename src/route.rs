use super::components::colophon::Colophon;
use super::components::home::Home;
use super::components::login::Login;
use super::components::page_not_found::PageNotFound;
use super::components::template::Template;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Routable, Serialize)]
pub enum Route {
  #[layout(Template)]
  #[route("/")]
  Home {},
  #[route("/colophon")]
  Colophon {},
  #[route("/login")]
  Login {},
  #[end_layout]
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
