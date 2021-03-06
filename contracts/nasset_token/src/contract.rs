use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::ContractResult;
use crate::{commands, state::save_config_holder_contract};
use basset_vault::nasset_token::{InstantiateMsg, MigrateMsg};
use cw20_base::allowances::{execute_decrease_allowance, execute_increase_allowance};
use cw20_base::contract::instantiate as cw20_instantiate;
use cw20_base::contract::query as cw20_query;
use cw20_base::contract::{execute_update_marketing, execute_upload_logo};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg as TokenInstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    let config_holder_contract = deps.api.addr_validate(&msg.config_holder_contract)?;
    save_config_holder_contract(deps.storage, &config_holder_contract)?;

    cw20_instantiate(
        deps,
        env,
        info,
        TokenInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            decimals: msg.decimals,
            initial_balances: msg.initial_balances,
            mint: msg.mint,
            marketing: msg.marketing,
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
        ExecuteMsg::Transfer { recipient, amount } => {
            commands::transfer(deps, env, info, recipient, amount)
        }

        ExecuteMsg::Burn { amount } => commands::burn(deps, env, info, amount),

        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => commands::send(deps, env, info, contract, amount, msg),

        ExecuteMsg::Mint { recipient, amount } => {
            commands::mint(deps, env, info, recipient, amount)
        }

        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),

        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),

        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => commands::transfer_from(deps, env, info, owner, recipient, amount),

        ExecuteMsg::BurnFrom { owner, amount } => {
            commands::burn_from(deps, env, info, owner, amount)
        }

        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => commands::send_from(deps, env, info, owner, contract, amount, msg),

        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => Ok(execute_update_marketing(
            deps,
            env,
            info,
            project,
            description,
            marketing,
        )?),

        ExecuteMsg::UploadLogo(logo) => Ok(execute_upload_logo(deps, env, info, logo)?),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    cw20_query(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
