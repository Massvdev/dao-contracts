#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, OverflowError, Response, StdError, StdResult, Storage,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Delegation, CONFIG, DELEGATION_COUNT};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cwd-proposal-delegate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const DEFAULT_POLICY_IRREVOCABLE: bool = false;
const DEFAULT_POLICY_PRESERVE_ON_FAILURE: bool = false;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;

    CONFIG.save(deps.storage, &Config { admin })?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Delegate {
            delegate,
            msgs,
            expiration,
            policy_irrevocable,
            policy_preserve_on_failure,
        } => {
            let policy_irrevocable = policy_irrevocable.unwrap_or(DEFAULT_POLICY_IRREVOCABLE);
            let policy_preserve_on_failure =
                policy_preserve_on_failure.unwrap_or(DEFAULT_POLICY_PRESERVE_ON_FAILURE);
            let delegate = deps.api.addr_validate(&delegate)?;
            execute_delegate(
                deps,
                env,
                info,
                Delegation {
                    delegate,
                    msgs,
                    expiration,
                    policy_irrevocable,
                    policy_preserve_on_failure,
                },
            )
        }
        ExecuteMsg::RemoveDelegation { delegation_id } => {
            execute_remove_delegation(deps, env, info, delegation_id)
        }
        ExecuteMsg::Execute { delegation_id } => execute_execute(deps, env, info, delegation_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

// MARK: Execute subroutines

fn execute_delegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    delegation: Delegation,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_remove_delegation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    delegation_id: u64,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    delegation_id: u64,
) -> Result<Response, ContractError> {
    unimplemented!()
}

// MARK: Helpers

fn advance_delegate_id(store: &mut dyn Storage) -> StdResult<u64> {
    let lhs = DELEGATION_COUNT.may_load(store)?.unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            DELEGATION_COUNT.save(store, &id)?;
            Ok(id)
        }
        None => Err(StdError::Overflow {
            source: OverflowError {
                operation: cosmwasm_std::OverflowOperation::Add,
                operand1: lhs.to_string(),
                operand2: 1.to_string(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {}
