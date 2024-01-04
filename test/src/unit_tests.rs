use std::collections::BTreeMap;

use rhaki_cw_plus::{
    serde_value::{json, ToCwJson, Value},
    traits::{IntoAddr, IntoBinary},
};
use variable_manager_pkg::definitions::Variable;

#[test]
#[rustfmt::skip]
fn parse_btreemap() {

    let mut map: BTreeMap<String, Variable> = BTreeMap::new();

    map.insert("1".to_string(), Variable::Addr("addr1".into_unchecked_addr()));

    map.insert("2".to_string(), Variable::Binary(json!({"key_1": "value_1", "key_2": "value_2"}).into_binary().unwrap()));

    let ser = serde_json::to_vec(&map).unwrap();

    let des: Value = serde_json::from_slice(&ser).unwrap();

    assert_eq!(des, json!({
        "1": {"addr": "addr1"},
        "2": {"binary": json!({"key_1": "value_1", "key_2": "value_2"}).into_binary().unwrap()}}).into_cw().unwrap());

    let des: BTreeMap<String, Variable> = serde_json::from_slice(&ser).unwrap();

    assert_eq!(des, map)

}
