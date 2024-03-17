use super::super::route::Route;
use super::callback::callback_query_segments::CallbackQuerySegments;
use crate::components::login_logout::calc_has_token_response;
use crate::components::template::TokenState;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  let use_shared_state_token_state_option: Option<&UseSharedState<TokenState>> =
    use_shared_state::<TokenState>(cx);

  let has_token_response: bool =
    calc_has_token_response(use_shared_state_token_state_option);

  let display: &str = if has_token_response {
    "block"
  } else {
    "none"
  };

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
          display: display,
          Link {
            to: Route::Colophon {},
            "Profile"
          }
        }
      }
    }
  }
}
