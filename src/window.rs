use crate::log::LogId;
// TODO: use gloo_util for Window instead?
use ::web_sys::Window;

pub fn get_origin() -> Option<String> {
  let window: Window = ::web_sys::window()?;

  let origin: String = window.origin();

  Some(origin)
}

pub fn log_info_origin() {
  let Some(origin) = get_origin() else {
    return;
  };

  log::info!("{} Window origin: {origin}", LogId::L019);
}
