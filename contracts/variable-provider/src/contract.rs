use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use rhaki_cw_plus::traits::{IntoAddr, IntoBinaryResult};
use variable_provider_pkg::{
    definitions::Config,
    msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use crate::{
    execute::{run_register_variable, run_remove_variable, run_update_owner_msg},
    query::{qy_get_all_variables, qy_get_variable, qy_get_variables},
    response::ContractResponse,
    state::CONFIG,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResponse {
    CONFIG.save(
        deps.storage,
        &Config {
            owners: msg
                .owners
                .iter()
                .map(|owner| -> StdResult<Addr> { owner.into_addr(deps.api) })
                .collect::<StdResult<Vec<Addr>>>()?,
        },
    )?;

    Ok(Response::new().add_attribute("owners", format!("{:?}", msg.owners)))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResponse {
    CONFIG.load(deps.storage)?.validate_owner(&info.sender)?;
    match msg {
        ExecuteMsg::RegisterVariable(msg) => run_register_variable(deps, msg),
        ExecuteMsg::RemoveVariable(msg) => run_remove_variable(deps, msg),
        ExecuteMsg::UpdateOwners(msg) => run_update_owner_msg(deps, msg),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetVariable { key } => qy_get_variable(deps, key).into_binary(),
        QueryMsg::GetVariables { keys } => qy_get_variables(deps, keys).into_binary(),
        QueryMsg::AllVariables { start_after, limit } => {
            qy_get_all_variables(deps, start_after, limit).into_binary()
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> ContractResponse {
    Ok(Response::new())
}
