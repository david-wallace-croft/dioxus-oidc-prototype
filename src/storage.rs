use crate::components::login_logout::constants;
use crate::log::LogId;
use gloo_storage::errors::StorageError;
use gloo_storage::{LocalStorage, Storage};
// TODO: use gloo_util for Window instead?
use ::web_sys::Window;

pub fn get_window_origin() -> Option<String> {
  let window: Window = ::web_sys::window()?;

  let origin: String = window.origin();

  Some(origin)
}

pub fn log_info_window_origin() {
  let Some(origin) = get_window_origin() else {
    return;
  };

  log::info!("{} Window origin: {origin}", LogId::L019);
}

pub fn pkce_verifier_delete() {
  log_info_window_origin();
  log::info!("{} Deleting PKCE verifier from storage...", LogId::L018);
  LocalStorage::delete(constants::STORAGE_KEY_PKCE_VERIFIER);
}

pub fn pkce_verifier_get() -> Option<String> {
  log_info_window_origin();
  log::info!("{} Load PKCE verifier from storage...", LogId::L004);
  let pkce_verifier_result: Result<String, StorageError> =
    LocalStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);
  match pkce_verifier_result {
    Ok(pkce_verifier) => {
      log::info!("{} PKCE verifier: {pkce_verifier}", LogId::L005);
      return Some(pkce_verifier);
    },
    Err(error) => {
      log::error!("{} Error: {error}", LogId::L006);
    },
  };
  None
}

pub fn pkce_verifier_set(pkce_verifier: &str) {
  log_info_window_origin();
  let result: Result<(), StorageError> =
    LocalStorage::set(constants::STORAGE_KEY_PKCE_VERIFIER, pkce_verifier);
  match result {
    Ok(_) => {
      log::info!("{} PKCE Verifier stored successfully", LogId::L016)
    },
    Err(storage_error) => {
      log::error!("{} {storage_error}", LogId::L017);
    },
  };
}
