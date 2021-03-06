use cosmwasm_std::{
    entry_point, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};

use crate::{
    commands,
    error::ContractError,
    queries,
    state::{load_config, save_config, save_state, State},
};
use crate::{state::Config, ContractResult};
use basset_vault::nasset_token_rewards::{
    AnyoneMsg, ExecuteMsg, GovernanceMsg, InstantiateMsg, MigrateMsg, QueryMsg, TokenMsg,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    let config = Config {
        psi_token: deps.api.addr_validate(&msg.psi_token_addr)?,
        nasset_token: deps.api.addr_validate(&msg.nasset_token_addr)?,
        governance_contract: deps.api.addr_validate(&msg.governance_contract_addr)?,
    };

    save_config(deps.storage, &config)?;
    save_state(
        deps.storage,
        &State {
            global_index: Decimal::zero(),
            total_balance: Uint128::zero(),
            prev_reward_balance: Uint128::zero(),
        },
    )?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Anyone { anyone_msg } => match anyone_msg {
            AnyoneMsg::UpdateGlobalIndex {} => commands::update_global_index(deps, env),

            AnyoneMsg::ClaimRewards { recipient } => {
                commands::claim_rewards(deps, env, info, recipient)
            }

            AnyoneMsg::ClaimRewardsForSomeone { address } => {
                commands::claim_rewards_for_someone(deps, env, address)
            }

            AnyoneMsg::AcceptGovernance {} => commands::accept_governance(deps, env, info),
        },

        ExecuteMsg::Token { token_msg } => {
            let config = load_config(deps.storage)?;
            if info.sender != config.nasset_token {
                return Err(ContractError::Unauthorized);
            }

            match token_msg {
                TokenMsg::IncreaseBalance { address, amount } => {
                    commands::increase_balance(deps, env, &config, address, amount)
                }

                TokenMsg::DecreaseBalance { address, amount } => {
                    commands::decrease_balance(deps, env, &config, address, amount)
                }
            }
        }

        ExecuteMsg::Governance { governance_msg } => {
            let config = load_config(deps.storage)?;
            if info.sender != config.governance_contract {
                return Err(ContractError::Unauthorized);
            }

            match governance_msg {
                GovernanceMsg::UpdateConfig {
                    psi_token_contract_addr,
                    nasset_token_contract_addr,
                } => commands::update_config(
                    deps,
                    config,
                    psi_token_contract_addr,
                    nasset_token_contract_addr,
                ),

                GovernanceMsg::UpdateGovernanceContract {
                    gov_addr,
                    seconds_to_wait_for_accept_gov_tx,
                } => commands::update_governance_addr(
                    deps,
                    env,
                    gov_addr,
                    seconds_to_wait_for_accept_gov_tx,
                ),
            }
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&queries::query_config(deps)?),
        QueryMsg::State {} => to_binary(&queries::query_state(deps)?),
        QueryMsg::AccruedRewards { address } => {
            to_binary(&queries::query_accrued_rewards(deps, address)?)
        }
        QueryMsg::Holder { address } => to_binary(&queries::query_holder(deps, env, address)?),
        QueryMsg::Holders {
            start_after,
            limit,
            order_by,
        } => to_binary(&queries::query_holders(deps, start_after, limit, order_by)?),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
