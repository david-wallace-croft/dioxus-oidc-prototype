use super::callback_query_segments::CallbackQuerySegments;
use crate::route::Route;
use ::com_croftsoft_lib_role::Validator;
use ::openidconnect::CsrfToken;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CallbackState {
  pub code_option: Option<String>,
  pub state_option: Option<String>,
}

impl From<&CallbackQuerySegments> for CallbackState {
  fn from(callback_query_segments: &CallbackQuerySegments) -> Self {
    let CallbackQuerySegments {
      code,
      state,
    } = callback_query_segments;
    let code_option = if code.is_empty() {
      None
    } else {
      Some(code.clone())
    };
    let state_option = if state.is_empty() {
      None
    } else {
      Some(state.clone())
    };
    Self {
      code_option,
      state_option,
    }
  }
}

impl Validator<bool> for CallbackState {
  fn validate(&self) -> bool {
    let mut valid = true;
    let CallbackState {
      code_option,
      ..
    } = self;
    if code_option.is_none() {
      log::info!("Invalid callback code");
      valid = false;
    } else {
      let code: String = code_option.clone().unwrap();
      log::info!("Callback code: {code}");
    }
    // if state_option.is_none() {
    //   log::info!("Invalid callback state");
    //   valid = false;
    // } else {
    //   let state: String = state_option.clone().unwrap();
    //   log::info!("Callback state: {state}");
    // }
    valid
  }
}

pub struct CallbackStateString(pub String);

const CALLBACK: &str = "callback";
const COLOPHON: &str = "colophon";

const HOME: &str = "home";

impl From<CallbackStateString> for Route {
  fn from(callback_state_string: CallbackStateString) -> Self {
    match callback_state_string.0.as_str() {
      CALLBACK => Route::Callback {
        query_params: CallbackQuerySegments::default(),
      },
      COLOPHON => Route::Colophon {},
      _ => Route::Home {},
    }
  }
}

impl From<Route> for CsrfToken {
  fn from(route: Route) -> Self {
    match route {
      Route::Callback {
        query_params: _,
      } => to_csrf_token(CALLBACK),
      Route::Colophon {} => to_csrf_token(COLOPHON),
      _ => to_csrf_token(HOME),
    }
  }
}

fn to_csrf_token(s: &str) -> CsrfToken {
  let wrapped_in_quotes: &str = &format!("\"{s}\"");
  ::serde_json::from_str(wrapped_in_quotes).unwrap()
}
