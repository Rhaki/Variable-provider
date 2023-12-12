pub mod msgs {
    use std::collections::BTreeMap;

    use cosmwasm_schema::{cw_serde, QueryResponses};
    use cosmwasm_std::Addr;

    use super::definitions::Variable;

    #[cw_serde]
    pub struct InstantiateMsg {
        pub owner: String,
    }

    #[cw_serde]
    pub enum ExecuteMsg {
        RegisterVariable(RegisterVariableMsg),
        RemoveVariable(RemoveVariableMsg),
    }

    #[cw_serde]
    pub struct RegisterVariableMsg {
        pub key: String,
        pub value: Variable,
    }

    #[cw_serde]
    pub struct RemoveVariableMsg {
        pub key: String,
    }

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {
        #[returns(Addr)]
        GetVariable { key: String },
        #[returns(BTreeMap<String, Variable>)]
        GetVariables { keys: Vec<String> },
        #[returns(Vec<(String, Variable)>)]
        AllVariables {
            start_after: Option<String>,
            limit: Option<u32>,
        },
    }

    #[cw_serde]
    pub struct MigrateMsg {}
}

pub mod definitions {

    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{from_json, Addr, Binary, Decimal, Deps, StdError, StdResult, Uint128};
    use rhaki_cw_plus::traits::AssertOwner;
    use serde::de::DeserializeOwned;

    #[cw_serde]
    pub struct Config {
        pub owner: Addr,
    }

    impl AssertOwner for Config {
        fn get_admin(&self) -> Addr {
            self.owner.clone()
        }
    }

    #[cw_serde]
    pub enum Variable {
        String(String),
        Addr(Addr),
        Uint128(Uint128),
        U64(u64),
        Decimal(Decimal),
        Binary(Binary),
    }

    impl Variable {
        pub fn unwrap_string(&self) -> StdResult<String> {
            if let Variable::String(val) = self {
                Ok(val.clone())
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not String, {:?}",
                    self
                )))
            }
        }

        pub fn unwrap_addr(&self) -> StdResult<Addr> {
            if let Variable::Addr(val) = self {
                Ok(val.clone())
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not Addr, {:?}",
                    self
                )))
            }
        }

        pub fn unwrap_uint128(&self) -> StdResult<Uint128> {
            if let Variable::Uint128(val) = self {
                Ok(*val)
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not Uint128, {:?}",
                    self
                )))
            }
        }

        pub fn unwrap_u64(&self) -> StdResult<u64> {
            if let Variable::U64(val) = self {
                Ok(*val)
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not u64, {:?}",
                    self
                )))
            }
        }

        pub fn unwrap_decimal(&self) -> StdResult<Decimal> {
            if let Variable::Decimal(val) = self {
                Ok(*val)
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not Decimal, {:?}",
                    self
                )))
            }
        }

        pub fn unwrap_binary<T: DeserializeOwned>(&self) -> StdResult<T> {
            if let Variable::Binary(val) = self {
                from_json::<T>(val)
            } else {
                Err(StdError::generic_err(format!(
                    "Variable is not Decimal, {:?}",
                    self
                )))
            }
        }

        pub fn validate(self, deps: Deps) -> StdResult<Variable> {
            if let Variable::Addr(val) = &self {
                deps.api.addr_validate(val.as_ref())?;
            }

            Ok(self)
        }
    }
}

pub mod helper {
    use std::collections::BTreeMap;

    use cosmwasm_std::{Deps, QuerierWrapper, StdResult};

    use crate::definitions::Variable;

    use super::msgs::QueryMsg;

    pub fn variable_provider_get_variable(
        querier: QuerierWrapper,
        key: impl Into<String>,
        variable_provider_addr: impl Into<String>,
    ) -> StdResult<Variable> {
        querier.query_wasm_smart(
            variable_provider_addr,
            &QueryMsg::GetVariable { key: key.into() },
        )
    }

    pub fn variable_provider_get_variables(
        deps: Deps,
        keys: Vec<impl Into<String>>,
        address_provider_addr: impl Into<String>,
    ) -> StdResult<BTreeMap<String, Variable>> {
        deps.querier.query_wasm_smart(
            address_provider_addr,
            &QueryMsg::GetVariables {
                keys: keys.into_iter().map(|val| val.into()).collect(),
            },
        )
    }
}
