use crate::components::login_logout::constants;
use crate::log::LogId;
use ::gloo_storage::errors::StorageError;
use ::gloo_storage::{LocalStorage, Storage};
use ::openidconnect::core::CoreTokenResponse;

pub fn location_get() -> Option<String> {
  log::info!("{} Load Location from storage...", LogId::L023);

  let location_result: Result<String, StorageError> =
    LocalStorage::get(constants::STORAGE_KEY_LOCATION);

  match location_result {
    Ok(location) => {
      log::info!("{} Location: {location}", LogId::L024);

      Some(location)
    },
    Err(error) => {
      log::error!("{} Error: {error}", LogId::L025);

      None
    },
  }
}

pub fn location_set(location: &str) {
  let result: Result<(), StorageError> =
    LocalStorage::set(constants::STORAGE_KEY_LOCATION, location);

  match result {
    Ok(_) => {
      log::info!("{} Location stored successfully", LogId::L021)
    },
    Err(storage_error) => {
      log::error!("{} {storage_error}", LogId::L022);
    },
  };
}

pub fn pkce_verifier_delete() {
  log::info!("{} Deleting PKCE verifier from storage...", LogId::L018);
  LocalStorage::delete(constants::STORAGE_KEY_PKCE_VERIFIER);
}

pub fn pkce_verifier_get() -> Option<String> {
  log::info!("{} Load PKCE verifier from storage...", LogId::L004);

  let pkce_verifier_result: Result<String, StorageError> =
    LocalStorage::get(constants::STORAGE_KEY_PKCE_VERIFIER);

  match pkce_verifier_result {
    Ok(pkce_verifier) => {
      log::info!("{} PKCE verifier: {pkce_verifier}", LogId::L005);

      Some(pkce_verifier)
    },
    Err(error) => {
      log::error!("{} Error: {error}", LogId::L006);

      None
    },
  }
}

pub fn pkce_verifier_set(pkce_verifier: &str) {
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

pub fn token_response_delete() {
  log::info!("{} Deleting token response from storage...", LogId::L029);

  LocalStorage::delete(constants::STORAGE_KEY_TOKEN_RESPONSE);
}

// TODO: Use get_all() instead to prevent console error message
pub fn token_response_get() -> Option<CoreTokenResponse> {
  log::info!("{} Load token response from storage...", LogId::L030);

  let token_response_result: Result<CoreTokenResponse, StorageError> =
    LocalStorage::get(constants::STORAGE_KEY_TOKEN_RESPONSE);

  match token_response_result {
    Ok(token_response) => {
      log::info!("{} Token response: {token_response:#?}", LogId::L031);

      Some(token_response)
    },
    Err(error) => {
      log::error!("{} Error: {error}", LogId::L020);

      None
    },
  }
}

pub fn token_response_set(token_response: &CoreTokenResponse) {
  let result: Result<(), StorageError> =
    LocalStorage::set(constants::STORAGE_KEY_TOKEN_RESPONSE, token_response);

  match result {
    Ok(_) => {
      log::info!("{} Token response stored successfully", LogId::L027)
    },
    Err(storage_error) => {
      log::error!("{} {storage_error}", LogId::L028);
    },
  };
}
