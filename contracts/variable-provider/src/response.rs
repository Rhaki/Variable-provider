use variable_provider_pkg::definitions::Variable;
use cosmwasm_std::{Response, StdError};
use thiserror::Error;

pub type ContractResponse = Result<Response, ContractError>;

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
}
