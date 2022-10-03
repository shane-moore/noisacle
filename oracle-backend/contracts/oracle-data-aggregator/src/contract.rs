#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr, Order};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OracleUpdate, QueryMsg};
use crate::state::{
    HAS_SUBMITTED_IN_ROUND, IS_SELECTED_IN_ROUND, LATEST_ROUND, NODES, NOIS_PROXY, ORACLE_VALUES,
    THRESHOLD,
};
use nois::{shuffle, NoisCallback, ProxyExecuteMsg};

const CONTRACT_NAME: &str = "crates.io:oracle-data-aggregator";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::InvalidProxyAddress)?;
    let nodes = msg
        .nodes
        .into_iter()
        .map(|node| {
            deps.api
                .addr_validate(&node)
                .map_err(|_| ContractError::InvalidNodeAddress)
        })
        .collect::<Result<Vec<Addr>, ContractError>>()?;
    let threshold: u8 = msg.threshold.parse().unwrap();

    if threshold > nodes.len() as u8 {
        return Err(ContractError::InvalidThreshold);
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    NODES.save(deps.storage, &nodes)?;
    THRESHOLD.save(deps.storage, &threshold)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InitiateNewRound {} => execute_initiate_new_round(deps, env, info),
        ExecuteMsg::Receive { callback } => execute_receive(deps, env, info, callback),
        ExecuteMsg::AddOracleValue { update } => execute_add_oracle_price(deps, env, info, update),
    }
}

pub fn execute_initiate_new_round(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let nois_proxy = NOIS_PROXY.load(deps.storage)?;

    let last_round_height = LATEST_ROUND.load(deps.storage).unwrap_or_default();
    let is_next_round = env.block.height >= last_round_height + 10;
    if !is_next_round {
        return Err(ContractError::NotNextRound);
    }

    LATEST_ROUND.save(deps.storage, &env.block.height)?;

    let response = Response::new().add_message(WasmMsg::Execute {
        contract_addr: nois_proxy.into(),
        msg: to_binary(&ProxyExecuteMsg::GetNextRandomness {
            job_id: env.block.height.to_string(),
        })?,
        funds: vec![],
    });
    Ok(response)
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    callback: NoisCallback,
) -> Result<Response, ContractError> {
    let _proxy = NOIS_PROXY.load(deps.storage)?;

    // TODO add back in once Nois Network is working
    // ensure_eq!(info.sender, proxy, ContractError::UnauthorizedReceive);

    let randomness: [u8; 32] = callback
        .randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness)?;

    let mut nodes = NODES.load(deps.storage)?;
    let threshold = THRESHOLD.load(deps.storage)?;

    shuffle(randomness, &mut nodes);

    nodes.into_iter().take(threshold.into()).for_each(|addr| {
        IS_SELECTED_IN_ROUND
            .save(deps.storage, (&callback.job_id, &addr), &true)
            .unwrap();
    });

    Ok(Response::default())
}

pub fn execute_add_oracle_price(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    update: OracleUpdate,
) -> Result<Response, ContractError> {
    let is_selected = IS_SELECTED_IN_ROUND
        .may_load(deps.storage, (&update.round_id, &info.sender))
        .unwrap_or_default()
        .unwrap_or_default();
    if !is_selected {
        return Err(ContractError::UnauthorizedUpdate);
    }
    let has_submitted = HAS_SUBMITTED_IN_ROUND
        .may_load(deps.storage, (&update.round_id, &info.sender))
        .unwrap_or_default()
        .unwrap_or_default();
    if !has_submitted {
        return Err(ContractError::AlreadySubmitted);
    }

    update.values.clone().into_iter().for_each(|v| {
        v.parse::<u128>().expect("Invalid value");
    });

    let mut all_asset_current_values = ORACLE_VALUES
        .may_load(deps.storage, &update.round_id)?
        .unwrap_or_default();

    if all_asset_current_values.is_empty() {
        ORACLE_VALUES.save(
            deps.storage,
            &update.round_id,
            &update.values.chunks(1).map(|s| s.into()).collect(),
        )?;
    } else {
        let total_submission_count = THRESHOLD.load(deps.storage)?;
        let current_submission_count: u8 = all_asset_current_values.len() as u8 + 1;

        for (i, current_asset_values) in all_asset_current_values.iter_mut().enumerate() {
            let new_asset_value = update.values.get(i).unwrap();
            current_asset_values.push(new_asset_value.to_string());
        }
        ORACLE_VALUES.save(deps.storage, &update.round_id, &all_asset_current_values)?;

        let is_last_submission = current_submission_count == total_submission_count;
        if is_last_submission {
            // send IBC message to subscriber
        }
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHistoryOfRounds {} => to_binary(&query_history(deps)?),
        QueryMsg::QueryIsSelected { round_id, node } => {
            to_binary(&query_is_selected(deps, round_id, node)?)
        }
        QueryMsg::QueryAllValues {} => to_binary(&query_all_values(deps)?),
        QueryMsg::QueryLastRoundId {} => to_binary(&query_last_round_id(deps)?),
    }
}

fn query_is_selected(deps: Deps, round_id: String, node: Addr) -> StdResult<Option<bool>> {
    let outcome = IS_SELECTED_IN_ROUND.may_load(deps.storage, (&round_id, &node))?;
    Ok(outcome)
}

fn query_last_round_id(deps: Deps) -> StdResult<u64> {
    let last_round_id = LATEST_ROUND.load(deps.storage)?;
    Ok(last_round_id)
}

fn query_history(deps: Deps) -> StdResult<Vec<String>> {
    let out: Vec<String> = IS_SELECTED_IN_ROUND
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|((id, addr), is_selected)| format!("{id}:{addr}:{is_selected}")))
        .collect::<StdResult<_>>()?;
    Ok(out)
}

fn query_all_values(deps: Deps) -> StdResult<Vec<String>> {
    let out: Vec<String> = ORACLE_VALUES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            item.map(|(round_id, values)| {
                format!(
                    "{}:{:?}:{:?}",
                    round_id,
                    get_median_for_values(values.clone()),
                    values
                )
            })
        })
        .collect::<StdResult<_>>()?;
    Ok(out)
}

fn get_median_for_values(numbers: Vec<Vec<String>>) -> Vec<String> {
    numbers.into_iter().map(get_median).collect()
}

fn get_median(numbers: Vec<String>) -> String {
    let mut parsed_numbers = numbers
        .iter()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    parsed_numbers.sort();

    if parsed_numbers.len() % 2 == 0 {
        let middle = parsed_numbers.len() / 2;
        let median = (parsed_numbers[middle - 1] + parsed_numbers[middle]) / 2;
        return median.to_string();
    }

    let middle = parsed_numbers.len() / 2;
    parsed_numbers[middle].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{Empty, HexBinary, OwnedDeps};

    const CREATOR: &str = "creator";
    const PROXY_ADDRESS: &str = "the proxy of choice";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            nois_proxy: "address123".to_string(),
            nodes: vec!["addr1".to_string()],
            threshold: "1".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn instantiate_proxy() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            nois_proxy: PROXY_ADDRESS.to_string(),
            nodes: vec!["addr1".to_string()],
            threshold: "1".to_string(),
        };
        let info = mock_info(CREATOR, &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        deps
    }

    #[test]
    fn execute_initiate_new_round_works() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::InitiateNewRound {};
        let info = mock_info("guest", &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn proxy_cannot_bring_an_existing_job_id() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "round_1".to_string(),
                randomness: HexBinary::from_hex(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            },
        };
        let info = mock_info(PROXY_ADDRESS, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "round_1".to_string(),
                randomness: HexBinary::from_hex(
                    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                )
                .unwrap(),
            },
        };
        let info = mock_info(PROXY_ADDRESS, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert!(matches!(err, ContractError::JobIdAlreadyPresent));
    }

    #[test]
    fn execute_receive_fails_for_invalid_randomness() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "round_1".to_string(),
                randomness: HexBinary::from_hex("ffffffff").unwrap(),
            },
        };
        let info = mock_info(PROXY_ADDRESS, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert!(matches!(err, ContractError::InvalidRandomness));
    }

    #[test]
    fn players_cannot_request_an_existing_job_id() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "111".to_string(),
                randomness: HexBinary::from_hex(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            },
        };
        let info = mock_info(PROXY_ADDRESS, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::InitiateNewRound {};
        let info = mock_info("guest", &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::JobIdAlreadyPresent));
    }

    #[test]
    fn execute_receive_fails_for_wrong_sender() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "123".to_string(),
                randomness: HexBinary::from_hex(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            },
        };
        let info = mock_info("guest", &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::UnauthorizedReceive));
    }

    #[test]
    fn execute_receive_works() {
        let mut deps = instantiate_proxy();

        let msg = ExecuteMsg::Receive {
            callback: NoisCallback {
                job_id: "123".to_string(),
                randomness: HexBinary::from_hex(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            },
        };
        let info = mock_info(PROXY_ADDRESS, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
