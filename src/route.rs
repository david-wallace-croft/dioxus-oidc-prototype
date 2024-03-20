use super::components::callback::callback_query_segments::CallbackQuerySegments;
use super::components::callback::Callback;
use super::components::colophon::Colophon;
use super::components::home::Home;
use super::components::logged_out::LoggedOut;
use super::components::page_not_found::PageNotFound;
use super::components::profile::Profile;
use super::components::template::Template;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Routable, Serialize)]
pub enum Route {
  // TODO: Do we need this layout(Template)?
  #[layout(Template)]
  #[route("/")]
  Home {},
  #[route("/colophon")]
  Colophon {},
  #[route("/callback?:query_params")]
  Callback {
    query_params: CallbackQuerySegments,
  },
  #[route("/logged-out")]
  LoggedOut {},
  #[route("/profile")]
  Profile {},
  #[end_layout]
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
