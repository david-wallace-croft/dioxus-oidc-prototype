use crate::components::login_logout::constants;
use crate::log::LogId;
use ::gloo_storage::errors::StorageError;
use ::gloo_storage::{LocalStorage, Storage};
use ::openidconnect::core::CoreTokenResponse;
use ::serde_json::Value;
use serde_json::Value::Object;

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

pub fn token_response_get() -> Option<CoreTokenResponse> {
  log::info!("{} Load token response from storage...", LogId::L030);

  let get_all_result: Result<Value, StorageError> = LocalStorage::get_all();

  if get_all_result.is_err() {
    let error: StorageError = get_all_result.err().unwrap();

    log::error!("{} Error: {error}", LogId::L020);

    return None;
  }

  let get_all_value: Value = get_all_result.unwrap();

  log::info!("{} Storage: {get_all_value:#?}", LogId::L034);

  let Object(mut map) = get_all_value else {
    log::error!("{} Storage value is not an object", LogId::L035);

    return None;
  };

  let token_response_value_option: Option<Value> =
    map.remove(constants::STORAGE_KEY_TOKEN_RESPONSE);

  let Some(token_response_value) = token_response_value_option else {
    log::error!("{} Token response not found", LogId::L031);

    return None;
  };

  log::info!(
    "{} Token response value: {token_response_value:#?}",
    LogId::L036
  );

  let token_response_result: Result<CoreTokenResponse, serde_json::Error> =
    serde_json::from_value(token_response_value);

  if let Err(error) = token_response_result {
    log::error!("{} Error: {error}", LogId::L033);

    return None;
  }

  let token_response: CoreTokenResponse = token_response_result.unwrap();

  log::info!("{} Token response: {token_response:#?}", LogId::L038);

  Some(token_response)
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
