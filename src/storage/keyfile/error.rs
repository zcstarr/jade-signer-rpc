//! # Errors for storage of `Keyfiles`

use crate::keystore::SerializeError;
use rocksdb;
use serde_json;

use failure::{Fail};
use std::{error, fmt, io, str};
use jsonrpc_core::{Error as JSONRpcError,ErrorCode as JSONRpcErrorCode};

#[derive(Debug, Fail)]
pub enum KeystoreError {
    /// General storage error
    #[fail(display = "KeyFile storage error: {}", _0)]
    StorageError(String),

    /// `KeyFile` not found  
    #[fail(display = "Missing KeyFile for address: {}", _0)]
    NotFound(String),
}

impl From<rocksdb::Error> for KeystoreError {
    fn from(err: rocksdb::Error) -> Self {
        KeystoreError::StorageError(format!("Keyfile storage error: {}", err.to_string()))
    }
}

impl From<serde_json::Error> for KeystoreError {
    fn from(err: serde_json::Error) -> Self {
        KeystoreError::StorageError(err.to_string())
    }
}

impl From<SerializeError> for KeystoreError {
    fn from(err: SerializeError) -> Self {
        KeystoreError::StorageError(err.to_string())
    }
}

impl From<str::Utf8Error> for KeystoreError {
    fn from(err: str::Utf8Error) -> Self {
        KeystoreError::StorageError(err.to_string())
    }
}

impl From<io::Error> for KeystoreError {
    fn from(err: io::Error) -> Self {
        KeystoreError::StorageError(err.to_string())
    }
}

impl From<KeystoreError> for jsonrpc_core::Error {
    fn from(err: KeystoreError) -> JSONRpcError {
        JSONRpcError { code: JSONRpcErrorCode::InternalError, 
                      message: err.to_string(), 
                      data: None} 
    }
}

impl fmt::Display for KeystoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KeystoreError::StorageError(ref str) => write!(f, "KeyFile storage error: {}", str),
            KeystoreError::NotFound(ref str) => write!(f, "Missing KeyFile for address: {}", str),
        }
    }
}