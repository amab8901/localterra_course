use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub host: Addr,
    pub opponent: Addr,
    pub host_move: Option<GameMove>,
    pub opp_move: Option<GameMove>,
    pub result: Option<GameResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Rps {
    pub host_and_opp: (Addr, Addr),
    pub state: State,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SearchResults {
    pub host_and_opp: (Addr, Addr),
    pub state: State,
}

pub const STATE: Item<State> = Item::new("state");
pub const RPS: Map<(Addr, Addr), State> = Map::new("host_account");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameResult {
    HostWins,
    OpponentWins,
    Tie,
}
