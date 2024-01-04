use cosmwasm_std::{Addr, Response, StdError};
use thiserror::Error;
use variable_manager_pkg::definitions::Variable;

pub type ContractResponse = Result<Response, ContractError>;
pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Key {key} alredy registered for address {value:?}")]
    KeyAlredyRegistered { key: String, value: Variable },

    #[error("Key not found: {key}")]
    KeyNotFound { key: String },

    #[error("Empty update owner msg")]
    InvalidUpdateOwnerMsg,

    #[error("Address is alredy a owner: {addr}")]
    IsAlredyOwner { addr: Addr },

    #[error("Address is not a owner: {addr}")]
    IsNotOwner { addr: Addr },
}
