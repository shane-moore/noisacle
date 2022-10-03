use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const NOIS_PROXY: Item<Addr> = Item::new("nois_proxy");
pub const LATEST_ROUND: Item<u64> = Item::new("latest_round");

pub const NODES: Item<Vec<Addr>> = Item::new("nodes");
pub const THRESHOLD: Item<u8> = Item::new("threshold");
pub const IS_SELECTED_IN_ROUND: Map<(&str, &Addr), bool> = Map::new("is_selected_in_round");
pub const HAS_SUBMITTED_IN_ROUND: Map<(&str, &Addr), bool> = Map::new("is_selected_in_round");

pub const ORACLE_VALUES: Map<&str, Vec<Vec<String>>> = Map::new("oracle_values");
