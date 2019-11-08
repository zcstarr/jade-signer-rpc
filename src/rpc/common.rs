//! Common types for JSON RPC handlers
//!

use serde::{Deserialize, Serialize};

use super::Error;
use crate::util;
use crate::util::{ChainID};
use serde_json::Value;

/// Trait to access a common chain name and id params
///
pub trait CommonChainParams {
    fn get_chain_id(&self) -> Option<usize>;
}

/// Check correspondence between chain name and chain numerical ID
/// If succeed, returns tuple of chain name and chain id.
///
///
/// # Arguments
///
/// * p - trait object to access chain name and id
///
/// # Errors
///
/// Return `Error` if parameters does not match
///
pub fn extract_chain_params(p: &dyn CommonChainParams) -> Result<(String, ChainID), Error> {
    let id_param = p.get_chain_id();
    let id: ChainID;
    let name: String;
    if id_param.is_some() {
        id = id_param.unwrap() as ChainID;
        name = id.to_string();
    } else {
        return Err(Error::InvalidDataFormat(
            "Required chain id parameter".to_string(),
        ));
    }

    Ok((name, id))
}

fn check_chain_name(n: &str) -> Result<u8, Error> {
    match util::to_chain_id(n) {
        Some(id) => Ok(id),
        None => Err(Error::InvalidDataFormat(format!(
            "Invalid chain name: {}",
            n
        ))),
    }
}

fn check_chain_id(id: u8) -> Result<String, Error> {
    match util::to_chain_name(id) {
        Some(n) => Ok(n.to_string()),
        None => Err(Error::InvalidDataFormat(format!(
            "Invalid chain id: {}",
            id
        ))),
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SignTxParams<T, U> {
    Left(T, String),
    Right(U),
}

impl<T, U: Default> SignTxParams<T, U> {
    #[allow(dead_code)]
    pub fn into_right(self) -> U {
        match self {
            SignTxParams::Left(_, _) => U::default(),
            SignTxParams::Right(u) => u,
        }
    }
}

impl<T, U: Default> SignTxParams<(T, String), (T, String, U)> {
    pub fn into_full(self) -> (T, String, U) {
        match self {
            SignTxParams::Left((t, s), _) => (t, s, U::default()),
            SignTxParams::Right((t, s, u)) => (t, s, u),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SignParams<U> {
    Left(String, String, String),
    Right(U),
}

impl<U: Default> SignParams<U> {
    #[allow(dead_code)]
    pub fn into_right(self) -> U {
        match self {
            SignParams::Left(_, _, _) => U::default(),
            SignParams::Right(u) => u,
        }
    }
}

impl<U: Default> SignParams<(String, String, String, U)> {
    pub fn into_full(self) -> (String, String, String, U) {
        match self {
            SignParams::Left(t, p, s) => (t, p, s, U::default()),
            SignParams::Right((t, p, s, u)) => (t, p, s, u),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SignTypedDataParams<U> {
    Left(String, Value, String),
    Right(U),
}

impl<U: Default> SignTypedDataParams<U> {
    #[allow(dead_code)]
    pub fn into_right(self) -> U {
        match self {
            SignTypedDataParams::Left(_, _, _) => U::default(),
            SignTypedDataParams::Right(u) => u,
        }
    }
}

impl<U: Default> SignTypedDataParams<(String, Value, String, U)> {
    pub fn into_full(self) -> (String, Value, String, U) {
        match self {
            SignTypedDataParams::Left(t, p, s) => (t, p, s, U::default()),
            SignTypedDataParams::Right((t, p, s, u)) => (t, p, s, u),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T, U: Default> Either<T, U> {
    pub fn into_right(self) -> U {
        match self {
            Either::Left(_) => U::default(),
            Either::Right(u) => u,
        }
    }
}

impl<T, U: Default> Either<(T,), (T, U)> {
    pub fn into_full(self) -> (T, U) {
        match self {
            Either::Left((t,)) => (t, U::default()),
            Either::Right((t, u)) => (t, u),
        }
    }
}

#[derive(Deserialize)]
pub struct ShakeAccountAccount {
    pub address: String,
    pub old_passphrase: String,
    pub new_passphrase: String,
}

#[derive(Deserialize)]
pub struct UpdateAccountAccount {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct NewAccountAccount {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub passphrase: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAccountAccount {
    pub name: String,
    pub address: String,
    pub description: String,
    pub hardware: bool,
    pub is_hidden: bool,
}

#[derive(Deserialize, Default, Debug)]
pub struct ListAccountsAdditional {
    #[serde(default)]
    pub chain_id: Option<usize>,
    #[serde(default)]
    pub show_hidden: bool,
    #[serde(default)]
    pub hd_path: Option<String>,
}

impl CommonChainParams for ListAccountsAdditional {
    fn get_chain_id(&self) -> Option<usize> {
        self.chain_id
    }
}

#[derive(Deserialize)]
pub struct SelectedAccount {
    pub address: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct CommonAdditional {
    #[serde(default)]
    pub chain_id: Option<usize>,
}

impl CommonChainParams for CommonAdditional {

    fn get_chain_id(&self) -> Option<usize> {
        self.chain_id
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SignTxTransaction {
    pub from: String,
    pub to: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub data: String,
    pub nonce: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct SignTxAdditional {
    #[serde(default)]
    pub chain_id: Option<usize>,
    #[serde(default)]
    pub hd_path: Option<String>,
}

impl CommonChainParams for SignTxAdditional {
    fn get_chain_id(&self) -> Option<usize> {
        self.chain_id
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct FunctionParams {
    pub values: Vec<String>,
    pub types: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewMnemonicAccount {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub passphrase: String,
    pub mnemonic: String,
    pub hd_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_chain_params() {
        let params = CommonAdditional {
            chain_id: Some(61),
        };

        let (name, id) = extract_chain_params(&params).unwrap();
        assert_eq!(id, 61);
        assert_eq!(name, "61");
    }


    #[test]
    fn should_check_empty_chain_params() {
        let params = CommonAdditional {
            chain_id: None,
        };

        let res = extract_chain_params(&params);
        assert!(res.is_err());
    }
}
