use variable_provider_pkg::definitions::{Config, Variable};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config_key");

pub const VARIABLES: Map<String, Variable> = Map::new("variables_key");
