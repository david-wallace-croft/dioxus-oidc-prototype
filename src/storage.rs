use crate::components::login_logout::constants;
use crate::log::LogId;
use gloo_storage::errors::StorageError;
use gloo_storage::{SessionStorage, Storage};

pub fn pkce_verifier_delete() {
  log::info!("{} Deleting PKCE verifier from storage...", LogId::L018);
  SessionStorage::delete(constants::STORAGE_KEY_PKCE_VERIFIER);
}

pub fn pkce_verifier_get() -> Option<String> {
  log::info!("{} Load PKCE verifier from storage...", LogId::L004);
  let pkce_verifier_result: Result<String, StorageError> =
    SessionStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);
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
  let result: Result<(), StorageError> =
    SessionStorage::set(constants::STORAGE_KEY_PKCE_VERIFIER, pkce_verifier);
  match result {
    Ok(_) => {
      log::info!("{} PKCE Verifier stored successfully", LogId::L016)
    },
    Err(storage_error) => {
      log::error!("{} {storage_error}", LogId::L017);
    },
  };
}
