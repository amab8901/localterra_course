#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GamesByHostResponse, GamesByOppResponse, HostMoveResponse, HostResponse,
    InstantiateMsg, OppMoveResponse, OpponentResponse, QueryMsg, ResultResponse,
};
use crate::state::{Rps, State, RPS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:raffle-dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        host: msg.host,
        opponent: msg.opponent,
        host_move: None,
        opp_move: None,
        result: None,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("host", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeHost { new_host } => try_change_host(deps, info, new_host),
        ExecuteMsg::StartGame {
            host_and_opp,
            state,
        } => try_start_game(deps, info, host_and_opp, state),
    }
}

pub fn try_change_host(
    deps: DepsMut,
    info: MessageInfo,
    host: Addr,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.host {
            return Err(ContractError::Unauthorized {});
        }
        state.host = host;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "change_host"))
}

pub fn try_start_game(
    deps: DepsMut,
    info: MessageInfo,
    host_and_opp: (Addr, Addr),
    state: State,
) -> Result<Response, ContractError> {
    if info.sender != state.host {
        return Err(ContractError::Unauthorized {});
    }
    let rps = Rps {
        host_and_opp,
        state,
    };

    RPS.save(
        deps.storage,
        (
            deps.api.addr_validate("host_lol").unwrap(),
            deps.api.addr_validate("opp_lol").unwrap(),
        ),
        &rps.state,
    )
    .unwrap();

    Ok(Response::new().add_attribute("method", "start_game"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHost {} => to_binary(&query_host(deps)?),
        QueryMsg::GetOpponent {} => to_binary(&query_opponent(deps)?),
        QueryMsg::GetHostMove {} => to_binary(&query_host_move(deps)?),
        QueryMsg::GetOpponentMove {} => to_binary(&query_opponent_move(deps)?),
        QueryMsg::GetResult {} => to_binary(&query_result(deps)?),
        QueryMsg::QueryGamesByHost { host } => to_binary(&query_games_by_host(deps, host)?),
        QueryMsg::QueryGamesByOpp { opp } => to_binary(&query_games_by_opp(deps, opp)?),
    }
}

fn query_host(deps: Deps) -> StdResult<HostResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(HostResponse { host: state.host })
}

fn query_opponent(deps: Deps) -> StdResult<OpponentResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OpponentResponse {
        opponent: state.opponent,
    })
}

fn query_host_move(deps: Deps) -> StdResult<HostMoveResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(HostMoveResponse {
        host_move: state.host_move,
    })
}

fn query_opponent_move(deps: Deps) -> StdResult<OppMoveResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OppMoveResponse {
        opp_move: state.opp_move,
    })
}

fn query_result(deps: Deps) -> StdResult<ResultResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(ResultResponse {
        result: state.result,
    })
}

fn query_games_by_host(deps: Deps, host: Addr) -> StdResult<GamesByHostResponse> {
    println!("before");

    let _x = RPS.prefix(host);
    dbg!(_x);
    
    println!("after");

    Ok(GamesByHostResponse { games: None })
}

fn query_games_by_opp(_deps: Deps, _opp: Addr) -> StdResult<GamesByOppResponse> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    use crate::state::{GameMove, GameResult};

    fn game_result(msg: InstantiateMsg) -> Option<GameResult> {
        match (msg.host_move.unwrap(), msg.opp_move.unwrap()) {
            (GameMove::Rock, GameMove::Rock) => Some(GameResult::Tie),
            (GameMove::Rock, GameMove::Paper) => Some(GameResult::OpponentWins),
            (GameMove::Rock, GameMove::Scissors) => Some(GameResult::HostWins),
            (GameMove::Paper, GameMove::Rock) => Some(GameResult::HostWins),
            (GameMove::Paper, GameMove::Paper) => Some(GameResult::Tie),
            (GameMove::Paper, GameMove::Scissors) => Some(GameResult::OpponentWins),
            (GameMove::Scissors, GameMove::Rock) => Some(GameResult::OpponentWins),
            (GameMove::Scissors, GameMove::Paper) => Some(GameResult::HostWins),
            (GameMove::Scissors, GameMove::Scissors) => Some(GameResult::Tie),
        }
    }

    #[test]
    fn change_host() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            host: Addr::unchecked("creator"),
            opponent: Addr::unchecked("opponent123"),
            host_move: Some(GameMove::Rock),
            opp_move: Some(GameMove::Scissors),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::ChangeHost {
            new_host: Addr::unchecked("Host Moe"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should change name
        let host_response: HostResponse =
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::GetHost {}).unwrap()).unwrap();
        assert_eq!(host_response.host, Addr::unchecked("Host Moe"));
    }

    #[test]
    fn start_game() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            host: Addr::unchecked("host123"),
            opponent: Addr::unchecked("Opponent Carl"),
            host_move: Some(GameMove::Rock),
            opp_move: Some(GameMove::Scissors),
        };
        let info = mock_info("creator2", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

        // execute message
        let info = mock_info("host123", &coins(2, "token"));
        let state = State {
            host: msg.host.clone(),
            opponent: msg.opponent.clone(),
            host_move: msg.host_move.clone(),
            opp_move: msg.opp_move.clone(),
            result: game_result(msg),
        };
        let msg = ExecuteMsg::StartGame {
            host_and_opp: (Addr::unchecked("host_lol"), Addr::unchecked("opp_lol")),
            state: state.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // query message
        let opp_response: OpponentResponse =
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::GetOpponent {}).unwrap())
                .unwrap();
        assert_eq!(opp_response.opponent, Addr::unchecked("Opponent Carl"));

        // result should be Host Wins
        assert_eq!(state.result, Some(GameResult::HostWins));
    }

    #[test]
    fn query_games() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            host: Addr::unchecked("host123"),
            opponent: Addr::unchecked("Opponent Carl"),
            host_move: Some(GameMove::Rock),
            opp_move: Some(GameMove::Scissors),
        };
        let info = mock_info("creator2", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

        let searched_host = Addr::unchecked("host123");
        let query_result = query_games_by_host(deps.as_ref(), searched_host);
        dbg!(&query_result);
    }
}
