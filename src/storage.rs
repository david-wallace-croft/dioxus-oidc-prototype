use crate::components::login_logout::constants;
use crate::log::LogId;
use crate::window;
use gloo_storage::errors::StorageError;
use gloo_storage::{LocalStorage, Storage};

pub fn pkce_verifier_delete() {
  window::log_info_origin();
  log::info!("{} Deleting PKCE verifier from storage...", LogId::L018);
  LocalStorage::delete(constants::STORAGE_KEY_PKCE_VERIFIER);
}

pub fn pkce_verifier_get() -> Option<String> {
  window::log_info_origin();
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
  window::log_info_origin();
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
