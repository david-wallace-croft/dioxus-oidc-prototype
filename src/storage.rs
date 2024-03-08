use crate::components::login_logout::constants;
use crate::log::LogId;
use ::gloo_storage::errors::StorageError;
use ::gloo_storage::{LocalStorage, Storage};
use ::openidconnect::core::CoreTokenResponse;
use ::serde::de::DeserializeOwned;
use ::serde_json::Value;
use ::serde_json::Value::Object;
use ::std::fmt::Debug;

#[derive(Debug)]
pub enum StorageKey {
  Location,
  PkceVerifier,
  TokenResponse,
}

impl StorageKey {
  fn to_constant(&self) -> &'static str {
    match self {
      StorageKey::Location => constants::STORAGE_KEY_LOCATION,
      StorageKey::PkceVerifier => constants::STORAGE_KEY_PKCE_VERIFIER,
      StorageKey::TokenResponse => constants::STORAGE_KEY_TOKEN_RESPONSE,
    }
  }
}

pub fn delete(storage_key: StorageKey) {
  log::trace!("{} Deleting {storage_key:?} from storage...", LogId::L018);

  let key: &str = storage_key.to_constant();

  LocalStorage::delete(key);
}

/// Gets a value from storage without showing a console error if not present.
pub fn get<T: Debug + DeserializeOwned>(storage_key: StorageKey) -> Option<T> {
  let get_all_result: Result<Value, StorageError> = LocalStorage::get_all();

  if get_all_result.is_err() {
    let error: StorageError = get_all_result.err().unwrap();

    log::error!("{} Error: {error}", LogId::L020);

    return None;
  }

  let get_all_value: Value = get_all_result.unwrap();

  log::debug!("{} Storage map: {get_all_value:#?}", LogId::L034);

  let Object(mut map) = get_all_value else {
    log::error!("{} Storage map is not an object", LogId::L035);

    return None;
  };

  let key: &str = storage_key.to_constant();

  let value: Value = map.remove(key)?;

  log::debug!("{} Storage value: {value:#?}", LogId::L036);

  let from_value_result: Result<T, serde_json::Error> =
    serde_json::from_value(value);

  if let Err(error) = from_value_result {
    log::error!("{} Error: {error}", LogId::L033);

    return None;
  }

  let deserialized: T = from_value_result.unwrap();

  log::debug!(
    "{} Storage value deserialized: {deserialized:#?}",
    LogId::L004
  );

  Some(deserialized)
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
