use super::callback_query_segments::CallbackQuerySegments;
use ::com_croftsoft_lib_role::Validator;

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
      state_option,
    } = self;
    if code_option.is_none() {
      log::info!("Invalid callback code");
      valid = false;
    } else {
      let code: String = code_option.clone().unwrap();
      log::info!("Callback code: {code}");
    }
    if state_option.is_none() {
      log::info!("Invalid callback state");
      valid = false;
    } else {
      let state: String = state_option.clone().unwrap();
      log::info!("Callback state: {state}");
    }
    valid
  }
}
