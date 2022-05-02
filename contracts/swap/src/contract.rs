#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info or migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, info, amount, msg),
    }
}

pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: i32,
    msg: Binary,
) -> Result<Response, ContractError> {
    // this doesn't work but the idea is good
    // if amount == i32::zero() {
    //     return Err(ContractError::InvalidZeroAmount {});
    // }

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count = count;
        Ok(state)
    })?;

    let res = Response::new()
        .add_attribute("method", "try_withdraw")
        .add_attribute("owner", info.sender)
        .add_attribute("amount", amount)
        .add_message(
            Cw20ReceiveMsg {
                sender: info.sender.into(),
                amount,
                msg,
            }
            .into_cosmos_msg(contract)?,
        );

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {

        //TODO
    }
}
