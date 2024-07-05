use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use crate::{error::ContractError, msg::{ExecuteMsg, InstantiateMsg}, state::{Config, CONFIG, OWNER, USER_INFO, USER_REQUEST}};

const NEW_REQ_REPLY_ID: u64 = 1;
mod exec;
mod reply;

//Instantiate will be used to set new requests to In Progress
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info:MessageInfo,
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
    use cosmwasm_std::{Deps, StdResult, Env, Binary, to_json_binary};

    use crate::{error::ContractError, msg::{UserInfoQueryResp, UserReqQueryResp, QueryMsg}, state::{USER_INFO, USER_REQUEST}};


    pub fn get_user_requests(deps: Deps, _env: Env, req_id: String) -> StdResult<Binary> {//StdResult<UserReqQueryResp>{
        // let value = USER_REQUEST.load(deps.storage, req_id)?;
        let value = USER_REQUEST.may_load(deps.storage, req_id)?;

        match value {
            Some(value) => {
                // let value = value.unwrap();
                let result = UserReqQueryResp{value};
                to_json_binary(&result)
            },
            None => {
                // StdError::
                // let no_existing_req_id = req_id.copy();
                Err(ContractError::NoExistingRequest{}.into())
            }
        }
        // Ok(UserReqQueryResp{value})
        // Ok(to_json_binary(&UserReqQueryResp{value}));
    }

    pub fn get_user_info(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {// StdResult<UserInfoQueryResp>{
        // let value = USER_INFO.load(deps.storage)?;
        let value = USER_INFO.may_load(deps.storage)?;

        match value {
            Some(value) => {
                // let value = value.unwrap();
                let result = UserInfoQueryResp{value};
                to_json_binary(&result)
            },
            None => {
                // StdError::
                Err(ContractError::NoExistingUser{}.into())
            }
        }

        // Ok(UserInfoQueryResp{value});
    }
}