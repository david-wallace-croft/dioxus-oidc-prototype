use crate::components::login::LoginQuerySegments;

use super::super::route::Route;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    nav {
      ul {
        li {
          Link {
            to: Route::Home {},
            "Home"
          }
        }
        li {
          Link {
            to: Route::Login {
              query_params: LoginQuerySegments {
                placeholder: String::new(),
              }
            },
            "Login"
          }
        }
        li {
          Link {
            to: Route::Colophon {},
            "Colophon"
          }
        }
      }
    }
  }
}
