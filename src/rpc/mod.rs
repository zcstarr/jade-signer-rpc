//! # JSON RPC module

mod common;
mod error;
mod serialize;
mod serves;

pub use self::error::Error;
use super::core;
use super::keystore::KdfDepthLevel;
use super::storage::{self, StorageController};
use super::util::{align_bytes, to_arr, to_even_str, to_u64, trim_hex, ToHex};
use jsonrpc_core::{Error as JsonRpcError, IoHandler, Params, Result};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use log::Level;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{self, Value};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use jade_signer_client::{RpcJadeSignerJSONRPCAPI, Address, Additional, Keyfile};

struct RpcServer {
  storage_ctrl: Arc<Mutex<StorageController>>,
    

}

impl RpcJadeSignerJSONRPCAPI for RpcServer {
        fn signer_importAccount(&mut self, keyfile: Keyfile, additional: Additional) -> Result<Address> {
            let storage_clone = Arc::clone(&self.storage_ctrl);
            let storage_ctrl = storage_clone.lock().unwrap();
            let storage = storage_ctrl.keyfile_storage;
            storage.put(&keyfile);
            log::debug!("Account imported: {}", keyfile.address);
            Ok(format!("{}", keyfile.address))
        }

    }



fn wrapper<T: Serialize>(value: Result<T, Error>) -> Result<Value, JsonRpcError> {
    if value.is_err() {
        return Err(JsonRpcError::invalid_params(
            value.err().unwrap().to_string(),
        ));
    }
    let value = value.unwrap();
    let result = serde_json::to_value(value);
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(JsonRpcError::invalid_params(e.to_string())),
    }
}

fn parse<T>(p: Params) -> Result<T, JsonRpcError>
where
    T: DeserializeOwned,
{
    p.parse()
        .map_err(|_| JsonRpcError::invalid_params("Corrupted input parameters".to_string()))
}

/// Start JSON-RPC server
///
/// # Arguments
///
/// * addr - socket address
/// * storage_ctrl - controller for `Keyfile` storage
/// * sec_level - security level
///
pub fn start(addr: &SocketAddr, storage_ctrl: StorageController, sec_level: Option<KdfDepthLevel>) {
    let sec_level = sec_level.unwrap_or_default();
    let storage_ctrl = Arc::new(Mutex::new(storage_ctrl));


    let mut io = IoHandler::default();

    {
        io.add_method("openrpc_discover", move |_: Params| {
            wrapper(serves::openrpc_discover())
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);

        io.add_method("signer_listAccounts", move |p: Params| {
            wrapper(serves::list_accounts(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_hideAccount", move |p: Params| {
            wrapper(serves::hide_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);

        io.add_method("signer_unhideAccount", move |p: Params| {
            wrapper(serves::unhide_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_shakeAccount", move |p: Params| {
            wrapper(serves::shake_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_updateAccount", move |p: Params| {
            wrapper(serves::update_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_importAccount", move |p: Params| {
            wrapper(serves::import_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_exportAccount", move |p: Params| {
            log::debug!("signer ------------- {:?}", p);
            wrapper(serves::export_account(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_newAccount", move |p: Params| {
            wrapper(serves::new_account(parse(p)?, sec_level, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_signTransaction", move |p: Params| {
            wrapper(serves::sign_transaction(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_sign", move |p: Params| {
            wrapper(serves::sign(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_signTypedData", move |p: Params| {
            wrapper(serves::sign_typed_data(parse(p)?, &storage_ctrl))
        });
    }

    {
        io.add_method("signer_encodeFunctionCall", move |p: Params| {
            wrapper(serves::encode_function_call(parse(p)?))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_listContracts", move |p: Params| {
            wrapper(serves::list_contracts(parse(p)?, &storage_ctrl))
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_importContract", move |p: Params| {
            wrapper(serves::import_contract(parse(p)?, &storage_ctrl))
        });
    }

    //    {
    //        let storage_ctrl = Arc::clone(&storage_ctrl);
    //        io.add_method("signer_exportContract", move |p: Params| {
    //            wrapper(serves::export_contract(parse(p)?, &storage_ctrl))
    //        });
    //    }

    {
        io.add_method("signer_generateMnemonic", move |_: Params| {
            wrapper(serves::generate_mnemonic())
        });
    }

    {
        let storage_ctrl = Arc::clone(&storage_ctrl);
        io.add_method("signer_importMnemonic", move |p: Params| {
            wrapper(serves::import_mnemonic(
                parse(p)?,
                &sec_level,
                &storage_ctrl,
            ))
        });
    }

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(addr)
        .expect("Expect to build HTTP RPC server");

    if log::log_enabled!(Level::Info) {
        log::info!("Connector started on http://{}", server.address());
    }

    server.wait();
}
