use cosmwasm_std::{attr, Addr, Attribute, Deps, DepsMut, Response};
use rhaki_cw_plus::traits::IntoAddr;
use variable_manager_pkg::{
    definitions::Config,
    msgs::{RegisterVariableMsg, RemoveVariableMsg, UpdateOwnerMsg},
};

use crate::{
    response::{ContractError, ContractResponse, ContractResult},
    state::{CONFIG, VARIABLES},
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
        .add_attribute("action", "register_variable")
        .add_attribute("key", msg.key)
        .add_attribute("value", format!("{}", msg.value)))
}

pub fn run_register_variables(
    mut deps: DepsMut,
    msgs: Vec<RegisterVariableMsg>,
) -> ContractResponse {
    let mut attrs = vec![attr("action", "register_variable")];
    for msg in msgs {
        run_register_variable(deps.branch(), msg.clone())?;
        attrs.push(attr("key", msg.key));
        attrs.push(attr("value", format!("{}", msg.value)));
    }

    Ok(Response::new().add_attributes(attrs))
}

pub fn run_update_variable(deps: DepsMut, msg: RegisterVariableMsg) -> ContractResponse {
    let validated = msg.value.clone().validate(deps.as_ref())?;

    VARIABLES.update(deps.storage, msg.key.clone(), |val| -> ContractResult<_> {
        val.ok_or(ContractError::KeyNotFound {
            key: msg.key.clone(),
        })?;
        Ok(validated)
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_variable")
        .add_attribute("key", msg.key)
        .add_attribute("value", format!("{}", msg.value)))
}

pub fn run_update_variables(mut deps: DepsMut, msgs: Vec<RegisterVariableMsg>) -> ContractResponse {
    let mut attrs = vec![attr("action", "update_variable")];
    for msg in msgs {
        run_update_variable(deps.branch(), msg.clone())?;
        attrs.push(attr("key", msg.key));
        attrs.push(attr("value", format!("{}", msg.value)));
    }

    Ok(Response::new().add_attributes(attrs))
}

pub fn run_remove_variable(deps: DepsMut, msg: RemoveVariableMsg) -> ContractResponse {
    let variable =
        VARIABLES
            .load(deps.storage, msg.key.clone())
            .map_err(|_| ContractError::KeyNotFound {
                key: msg.key.to_string(),
            })?;

    VARIABLES.remove(deps.storage, msg.key.clone());

    Ok(Response::new()
        .add_attribute("action", "remove_variable")
        .add_attribute("key", msg.key)
        .add_attribute("value", format!("{}", variable)))
}

pub fn run_update_owner_msg(deps: DepsMut, msg: UpdateOwnerMsg) -> ContractResponse {
    let mut config = CONFIG.load(deps.storage)?;

    let attrs = match (msg.add, msg.remove) {
        (None, None) => Err(ContractError::InvalidUpdateOwnerMsg),
        (None, Some(to_remove)) => remove_owners(deps.as_ref(), &mut config, to_remove),
        (Some(to_add), None) => add_owners(deps.as_ref(), &mut config, to_add),
        (Some(to_add), Some(to_remove)) => {
            let mut combined = vec![];
            combined.extend_from_slice(&add_owners(deps.as_ref(), &mut config, to_add)?);
            combined.extend_from_slice(&remove_owners(deps.as_ref(), &mut config, to_remove)?);
            Ok(combined)
        }
    }?;

    config.validate()?;

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "update_owners")
        .add_attributes(attrs))
}

fn add_owners(
    deps: Deps,
    config: &mut Config,
    addresses: Vec<String>,
) -> ContractResult<Vec<Attribute>> {
    let mut attrs: Vec<Attribute> = vec![];

    for address in addresses {
        let address = address.into_addr(deps.api)?;

        if config.owners.contains(&address) {
            return Err(ContractError::IsAlredyOwner { addr: address });
        }

        attrs.push(attr("owner_added", address.clone()));

        config.owners.push(address)
    }

    Ok(attrs)
}

fn remove_owners(
    deps: Deps,
    config: &mut Config,
    addresses: Vec<String>,
) -> ContractResult<Vec<Attribute>> {
    let mut attrs: Vec<Attribute> = vec![];

    let addresses = addresses
        .into_iter()
        .map(|address| -> ContractResult<Addr> {
            let address = address.into_addr(deps.api)?;

            if !config.owners.contains(&address) {
                return Err(ContractError::IsNotOwner { addr: address });
            }

            attrs.push(attr("owner_removed", address.clone()));

            Ok(address)
        })
        .collect::<ContractResult<Vec<Addr>>>()?;

    config.owners.retain_mut(|addr| !addresses.contains(addr));

    Ok(attrs)
}
