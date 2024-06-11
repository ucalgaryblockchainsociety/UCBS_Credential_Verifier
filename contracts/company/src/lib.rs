

use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
pub use crate::error::ContractError;
use msg::{InstantiateCompanyMsg, QueryCompanyMsg, RequestVerify};
use controller::msg::ExecuteMsg;

#[cfg(any(test, feature = "mt"))]
pub mod multitest;
pub mod contract;
mod error;
pub mod msg;
pub mod state;
pub mod query;

#[entry_point]
pub fn instantiate(
    deps:DepsMut,
    env:Env,
    _info: MessageInfo,
    msg: InstantiateCompanyMsg,
) -> Result<Response, ContractError>{
    contract::instantiate(deps, env, _info, msg)
}

#[entry_point]
pub fn execute(deps:DepsMut, env: Env, info:MessageInfo, msg:RequestVerify) ->Result<Response, ContractError>{

    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps:Deps, env: Env,msg:QueryCompanyMsg) ->StdResult<Binary>{

    contract::query(deps, env, msg)
}




// #[entry_point]
// pub fn reply(deps:DepsMut, env:Env, reply:Reply) -> Result<Response, ContractError>{
//     contract::reply(deps, env, reply)
// }
