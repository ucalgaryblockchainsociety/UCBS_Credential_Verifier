pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

// use contract::query;
use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps:DepsMut,
    env:Env,
    msg: InstantiateMsg,
) -> StdResult<Response>{
    contract::instantiate(deps,env,msg)
}


#[entry_point]
pub fn execute(deps:DepsMut, env: Env, info:MessageInfo, msg:ExecuteMsg) ->Result<Response, ContractError>{

    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn reply(deps:DepsMut, env:Env, reply:Reply) -> Result<Response, ContractError>{
    contract::reply(deps, env, reply)
}

// #[entry_point]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary>{
//     use crate::QueryMsg::{UserRequest,EmpRequests};
//     match msg{
//         UserRequests{user_id} => to_json_binary(&query::get_user_requests(deps, request_id)?)
//         UserInfo{} => to_json_binary(&query::get_user_info(deps)?)
//     }
// }
