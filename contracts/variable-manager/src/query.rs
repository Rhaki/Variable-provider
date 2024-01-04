use std::collections::BTreeMap;

use cosmwasm_std::{Deps, StdError, StdResult};
use variable_manager_pkg::definitions::Variable;

use crate::state::VARIABLES;

pub fn qy_get_variable(deps: Deps, key: String) -> StdResult<Variable> {
    VARIABLES
        .load(deps.storage, key.clone())
        .map_err(|_| StdError::generic_err(format!("variable not found - key: {key}")))
}

pub fn qy_get_variables(deps: Deps, keys: Vec<String>) -> StdResult<BTreeMap<String, Variable>> {
    keys.into_iter()
        .map(|key| {
            Ok((
                key.clone(),
                VARIABLES.load(deps.storage, key.clone()).map_err(|_| {
                    StdError::generic_err(format!("Variable not found - key: {key}"))
                })?,
            ))
        })
        .collect::<StdResult<BTreeMap<String, Variable>>>()
}

pub fn qy_get_all_variables(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<(String, Variable)>> {
    rhaki_cw_plus::storage::map::get_items(
        deps.storage,
        &VARIABLES,
        cosmwasm_std::Order::Ascending,
        limit,
        start_after,
    )
}
