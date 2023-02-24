use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{GameMove, GameResult, State};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub host: Addr,
    pub opponent: Addr,
    pub host_move: Option<GameMove>,
    pub opp_move: Option<GameMove>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ChangeHost {
        new_host: Addr,
    },
    StartGame {
        host_and_opp: (Addr, Addr),
        state: State,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetHost {},
    GetOpponent {},
    GetHostMove {},
    GetOpponentMove {},
    GetResult {},
    QueryGamesByHost { host: Addr },
    QueryGamesByOpp { opp: Addr },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HostResponse {
    pub host: Addr,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OpponentResponse {
    pub opponent: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HostMoveResponse {
    pub host_move: Option<GameMove>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OppMoveResponse {
    pub opp_move: Option<GameMove>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResultResponse {
    pub result: Option<GameResult>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GamesByHostResponse {
    pub games: Option<State>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GamesByOppResponse {
    pub games: Option<State>
}

