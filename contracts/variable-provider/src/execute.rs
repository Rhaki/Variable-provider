use variable_provider_pkg::msgs::{RegisterVariableMsg, RemoveVariableMsg};
use cosmwasm_std::{DepsMut, Response};

use crate::{
    response::{ContractError, ContractResponse},
    state::VARIABLES,
};

pub fn run_register_variable(deps: DepsMut, msg: RegisterVariableMsg) -> ContractResponse {
    if let Ok(variable) = VARIABLES.load(deps.storage, msg.key.clone()) {
        return Err(ContractError::KeyAlredyRegistered {
            key: msg.key,
            value: variable,
        });
    }

    let validate = msg.value.clone().validate(deps.as_ref())?;

    VARIABLES.save(deps.storage, msg.key.clone(), &validate)?;

    Ok(Response::new()
        .add_attribute("action", "register_address")
        .add_attribute("key", msg.key)
        .add_attribute("value", format!("{:?}", msg.value)))
}

pub fn run_remove_variable(deps: DepsMut, msg: RemoveVariableMsg) -> ContractResponse {
    let variable = VARIABLES.load(deps.storage, msg.key.clone())?;

    VARIABLES.remove(deps.storage, msg.key.clone());

    Ok(Response::new()
        .add_attribute("action", "remove_address")
        .add_attribute("key", msg.key)
        .add_attribute("value", format!("{:?}", variable)))
}
