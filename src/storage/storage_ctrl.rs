use super::contracts::ContractStorage;
use super::{build_keyfile_storage, build_path, KeyfileStorage};
use crate::storage::StorageType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use failure::{Error};

/// Controller to switch storage according to specified chain
pub struct StorageController {
    pub keyfile_storage: Box<dyn KeyfileStorage>,
    base_path: PathBuf,
    storage_type: StorageType,
}

impl StorageController {
    /// Create new `StorageController`
    /// with a subfolders for
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        storage_type: StorageType,
    ) -> Result<StorageController, Error> {
        log::debug!("STARTING PATH {:?}", base_path.as_ref().to_str());
        let storage =
            build_keyfile_storage(build_path(base_path.as_ref(), "keystore"), storage_type)?;
        Ok(StorageController {
            keyfile_storage: storage,
            base_path: PathBuf::from(base_path.as_ref()),
            storage_type: storage_type,
        })
    }

}
