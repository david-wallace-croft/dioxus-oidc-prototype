use crate::components::login_logout::constants;
use crate::log::LogId;
use ::gloo_storage::errors::StorageError;
use ::gloo_storage::{LocalStorage, Storage};
use ::serde::de::DeserializeOwned;
use ::serde::Serialize;
use ::serde_json::Value;
use ::serde_json::Value::Object;
use ::std::fmt::Debug;

#[derive(Debug)]
pub enum StorageKey {
  // TODO: Move this to a secure cookie?
  PkceVerifier,
}

impl StorageKey {
  fn to_constant(&self) -> &'static str {
    match self {
      StorageKey::PkceVerifier => constants::STORAGE_KEY_PKCE_VERIFIER,
    }
  }
}

pub fn delete(storage_key: StorageKey) {
  log::trace!("{} Deleting {storage_key:?} from storage...", LogId::L018);

  let key: &str = storage_key.to_constant();

  LocalStorage::delete(key);
}

// TODO: Move these functions to a trait?

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

// TODO: Can I force the value type to match the storage key type?
pub fn set<T: Debug + Serialize + ?Sized>(
  storage_key: StorageKey,
  value: &T,
) -> Result<(), StorageError> {
  log::trace!("{} Setting {storage_key:?} in storage...", LogId::L005);

  let key: &str = storage_key.to_constant();

  let result: Result<(), StorageError> = LocalStorage::set(key, value);

  match &result {
    Ok(_) => {
      log::debug!(
        "{} Setting {storage_key:?} in storage succeeded.",
        LogId::L021
      );
    },
    Err(storage_error) => {
      log::error!("{} {storage_error}", LogId::L022);
    },
  };

  result
}
