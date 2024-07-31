use {
    cosmwasm_std::{Addr, Binary},
    rhaki_cw_plus::{
        math::IntoDecimal,
        multi_test::helper::{
            anyhow::Result as AnyResult,
            build_bech32_app, create_code,
            cw_multi_test::{AppResponse, Executor},
            Bech32App, Bench32AppExt, UnwrapError,
        },
    },
    std::collections::BTreeMap,
    variable_manager_pkg::{
        definitions::{Config, Variable},
        msgs::{QueryMsg, RegisterVariableMsg, RemoveVariableMsg, UpdateOwnerMsg},
    },
};

const CHAIN_PREFIX: &str = "cosmos";

struct Def {
    pub owner: Addr,
    pub vm_addr: Addr,
}

fn startup() -> (Bech32App, Def) {
    let mut app = build_bech32_app(CHAIN_PREFIX);

    let owner = app.generate_addr("owner");

    let code_id = app.store_code(create_code(
        variable_manager::contract::instantiate,
        variable_manager::contract::execute,
        variable_manager::contract::query,
    ));

    let vm_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &variable_manager_pkg::msgs::InstantiateMsg {
                owners: vec![owner.to_string()],
            },
            &[],
            "vm",
            None,
        )
        .unwrap();

    (app, Def { owner, vm_addr })
}

fn register_variable(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    key: &str,
    variable: &Variable,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::RegisterVariable(RegisterVariableMsg {
            key: key.to_string(),
            value: variable.clone(),
        }),
        &[],
    )
}

fn register_variables(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    keys_values: Vec<(&str, &Variable)>,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::RegisterVariables(
            keys_values
                .into_iter()
                .map(|(k, v)| RegisterVariableMsg {
                    key: k.to_string(),
                    value: v.clone(),
                })
                .collect(),
        ),
        &[],
    )
}

fn remove_variable(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    key: &str,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::RemoveVariable(RemoveVariableMsg {
            key: key.to_string(),
        }),
        &[],
    )
}

fn update_variable(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    key: &str,
    variable: &Variable,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::UpdateVariable(RegisterVariableMsg {
            key: key.to_string(),
            value: variable.clone(),
        }),
        &[],
    )
}

fn update_variables(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    keys_values: Vec<(&str, &Variable)>,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::UpdateVariables(
            keys_values
                .into_iter()
                .map(|(k, v)| RegisterVariableMsg {
                    key: k.to_string(),
                    value: v.clone(),
                })
                .collect(),
        ),
        &[],
    )
}

fn update_owners(
    app: &mut Bech32App,
    def: &Def,
    sender: &Addr,
    add: Option<&[&Addr]>,
    remove: Option<&[&Addr]>,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        def.vm_addr.clone(),
        &variable_manager_pkg::msgs::ExecuteMsg::UpdateOwners(UpdateOwnerMsg {
            add: add.map(|add| add.iter().map(|a| a.to_string()).collect()),
            remove: remove.map(|remove| remove.iter().map(|a| a.to_string()).collect()),
        }),
        &[],
    )
}

fn qy_config(app: &Bech32App, def: &Def) -> AnyResult<Config> {
    Ok(app
        .wrap()
        .query_wasm_smart(&def.vm_addr, &QueryMsg::Config {})?)
}

fn qy_variable(app: &Bech32App, def: &Def, key: &str) -> AnyResult<Variable> {
    Ok(app.wrap().query_wasm_smart(
        &def.vm_addr,
        &QueryMsg::GetVariable {
            key: key.to_string(),
        },
    )?)
}

fn qy_variables(app: &Bech32App, def: &Def, key: &[&str]) -> AnyResult<BTreeMap<String, Variable>> {
    Ok(app.wrap().query_wasm_smart(
        &def.vm_addr,
        &QueryMsg::GetVariables {
            keys: key.into_iter().map(|val| val.to_string()).collect(),
        },
    )?)
}

fn qy_all_variables(
    app: &Bech32App,
    def: &Def,
    start_after: Option<&str>,
    limit: Option<u32>,
) -> AnyResult<Vec<(String, Variable)>> {
    Ok(app.wrap().query_wasm_smart(
        &def.vm_addr,
        &QueryMsg::AllVariables {
            start_after: start_after.map(|val| val.to_string()),
            limit,
        },
    )?)
}

#[test]
#[rustfmt::skip]
fn integration() {
    let (mut app, mut def) = startup();

    let mut var_1 = Variable::String("var_1".to_string());
    let mut var_2 = Variable::Uint128(100u128.into());
    let mut var_3 = Variable::Binary(Binary(vec![1, 2, 3, 4]));
    let var_4 = Variable::Decimal("1.5".into_decimal());

    let random_addr = app.generate_addr("random_addr");

    // --- Err Invalid owner ---
    {
        register_variable(&mut app, &def, &random_addr, "var_1", &var_1).unwrap_err_contains("not an owner");
    }

    // --- Ok  register variable ---
    {
        register_variable(&mut app, &def, &def.owner, "var_1", &var_1).unwrap();
        register_variable(&mut app, &def, &def.owner, "var_2", &var_2).unwrap();
    }

    // --- Ok  register variables ---
    {
        register_variables(&mut app, &def, &def.owner, vec![("var_3", &var_3), ("var_4", &var_4)]).unwrap();
    }

    // --- Err variable alredy registered ---
    {
        register_variable(&mut app, &def, &def.owner, "var_1", &var_1).unwrap_err_contains("already registered");
        register_variables(&mut app, &def, &def.owner, vec![("var_3", &var_3), ("var_4", &var_4)]).unwrap_err_contains("already registered");
    }

    // --- Ok  remove variable ---
    {
        remove_variable(&mut app, &def, &def.owner, "var_1").unwrap();
    }

    // --- Err remove unexisting variable ---
    {
        remove_variable(&mut app, &def, &def.owner, "var_5").unwrap_err_contains("Key not found: var_5");
    }

    // --- OK  update variable ---
    {
        register_variable(&mut app, &def, &def.owner, "var_1", &var_1).unwrap();
        var_1 = Variable::String("var_1_updated".to_string());
        update_variable(&mut app, &def, &def.owner, "var_1", &var_1).unwrap();
    }

    // --- Err update unexisting variable ---
    {
        update_variable(&mut app, &def, &def.owner, "var_10", &var_1).unwrap_err_contains("Key not found: var_10");
    }

    // --- OK  update variables ---
    {
        var_2 = Variable::Uint128(200u128.into());
        var_3 = Variable::Binary(Binary(vec![4, 3, 2, 1]));
        update_variables(&mut app, &def, &def.owner, vec![("var_2", &var_2), ("var_3", &var_3)]).unwrap(); 
    }

    // --- Err update unexisting variables ---
    {
        update_variables(&mut app, &def, &def.owner, vec![("var_20", &var_2), ("var_3", &var_3)]).unwrap_err_contains("Key not found: var_20"); 
    }

    // --- Err update owner ---
    {
        // sanity check
        update_owners(&mut app, &def, &random_addr, None, None).unwrap_err_contains("not an owner");
        
        update_owners(&mut app, &def, &def.owner, None, Some(&[&def.owner])).unwrap_err_contains("Invalid 0 owners. Needed at least 1");
        update_owners(&mut app, &def, &def.owner, Some(&[&def.owner]), None).unwrap_err_contains("is alredy a owner");
        let old_owner = def.owner.clone();
        def.owner = app.generate_addr("new_owner");
        update_owners(&mut app, &def, &old_owner, Some(&[&def.owner]), Some(&[&old_owner])).unwrap();
    }

    // --- Assert config ---
    {
        let config = qy_config(&mut app, &def).unwrap();
        assert_eq!(config.owners.len(), 1);
        assert_eq!(config.owners[0], def.owner);
    }

    // --- Assert variables ---
    {
        assert_eq!(var_1, qy_variable(&mut app, &def, "var_1").unwrap());
        assert_eq!(
            BTreeMap::from([("var_1".to_string(), var_1.clone()), ("var_2".to_string(), var_2.clone())]),
            qy_variables(&mut app, &def, &["var_1", "var_2"]).unwrap()
        );
        assert_eq!(
           vec![("var_1".to_string(), var_1.clone()), ("var_2".to_string(), var_2.clone())],
            qy_all_variables(&mut app, &def, None, Some(2)).unwrap()
        );
        assert_eq!(
            vec![("var_3".to_string(), var_3.clone()), ("var_4".to_string(), var_4.clone())],
             qy_all_variables(&mut app, &def, Some("var_2"), None).unwrap()
         );
    }

}
