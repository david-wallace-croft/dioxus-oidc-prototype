use super::super::route::Route;
use super::callback::callback_query_segments::CallbackQuerySegments;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    nav {
      class: "app-nav",
      ul {
        li {
          Link {
            to: Route::Home {},
            "Home"
          }
        }
        li {
          Link {
            to: Route::Callback {
              query_params: CallbackQuerySegments {
                code: String::new(),
                state: String::new(),
              }
            },
            "Callback"
          }
        }
        li {
          Link {
            to: Route::Colophon {},
            "Colophon"
          }
        }
        li {
          Link {
            to: Route::Profile {},
            "Profile"
          }
        }
      }
    }
  }
}
