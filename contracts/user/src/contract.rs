use cosmwasm_std::{ensure, to_json_binary, DepsMut, Env, MessageInfo, Reply, Response, StdResult, WasmMsg};
use crate::{error::ContractError, msg::{ControllerExecMsg, ExecuteMsg, InstantiateMsg, UserInfo, UserRequest}, state::{Config, CONFIG, OWNER, USER_INFO, USER_REQUEST}};

const NEW_REQ_REPLY_ID: u64 = 1;
mod exec;
mod reply;

//Instantiate will be used to set new requests to In Progress
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    let owner = deps.api.addr_validate(&msg.owner)?;
    let controller_contract = deps.api.addr_validate(&msg.controller_contract)?;
    
    OWNER.save(deps.storage, &owner)?;
    USER_INFO.save(deps.storage, &msg.user_info)?;
    USER_REQUEST.save(deps.storage, msg.user_info.user_id, &msg.user_requests)?;
    CONFIG.save(
        deps.storage,
        &Config{
            controller_contract,
            is_valid: false,
        },
    )?;

    Ok(Response::new())
}


pub fn execute(
    deps:DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
)->Result<Response, ContractError>{
    use ExecuteMsg::*;

    match msg{
        NewRequest{
            user_request,
        } => exec::newrequest(deps, env, info, user_request),
        UpdateRequest{
            update_request,
        } => exec::updaterequest(deps,env, info, update_request),
    }
}

pub fn reply(deps: DepsMut,env:Env,reply: Reply) -> Result<Response, ContractError>{
    match reply.id{
        NEW_REQ_REPLY_ID => reply::new_request(deps, env),
        id => Err(ContractError::UnrecognizedReplyID(id))
    }
}

pub mod query{
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::{UserInfoQueryResp, UserReqQueryResp}, state::{USER_INFO, USER_REQUEST}};


    pub fn get_user_requests(deps: Deps, req_id: String) -> StdResult<UserReqQueryResp>{
        let value = USER_REQUEST.load(deps.storage, req_id)?;
        Ok(UserReqQueryResp{value})
    }

    pub fn get_user_info(deps: Deps) -> StdResult<UserInfoQueryResp>{
        let value = USER_INFO.load(deps.storage)?;
        Ok(UserInfoQueryResp{value})
    }
}