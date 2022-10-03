use cosmwasm_std::Addr;
use nois::NoisCallback;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InstantiateMsg {
    pub nois_proxy: String,
    pub nodes: Vec<String>,
    pub threshold: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OracleUpdate {
    pub round_id: String,
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InitiateNewRound {},
    Receive { callback: NoisCallback },
    AddOracleValue { update: OracleUpdate },
}

// { "add_oracle_value": { "update": { "round_id": "1345286", "values": ["1","2","3","4"] } } }

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryIsSelected { round_id: String, node: Addr },
    QueryAllValues {},
    GetHistoryOfRounds {},
    QueryLastRoundId {},
}
