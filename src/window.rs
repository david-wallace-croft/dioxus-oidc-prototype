use crate::log::LogId;
use ::web_sys::wasm_bindgen::JsValue;
use ::web_sys::Location;
// TODO: use gloo_util for Window instead?
use ::web_sys::Window;

pub fn get_location() -> Option<String> {
  let window: Window = ::web_sys::window()?;

  let location: Location = window.location();

  let href_result: Result<String, JsValue> = location.href();

  href_result.ok()
}

pub fn get_origin() -> Option<String> {
  let window: Window = ::web_sys::window()?;

  let origin: String = window.origin();

  Some(origin)
}

// pub fn log_info_location() {
//   let Some(location) = get_location() else {
//     return;
//   };

//   log::info!("{} Window location: {location}", LogId::L020);
// }

pub fn log_info_origin() {
  let Some(origin) = get_origin() else {
    return;
  };

  log::info!("{} Window origin: {origin}", LogId::L019);
}
