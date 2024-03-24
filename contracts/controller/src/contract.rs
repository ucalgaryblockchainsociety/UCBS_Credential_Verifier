use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use crate::{error::ContractError, msg::{CompInfo, ExecuteMsg, InstantiateMsg, UserInfo}, state::{Config, COMPANIES, CONFIG, EMPLOYEES, OWNER}};


const COMP_NEW_REQ_REPLY_ID: u64 = 1;
const USER_UPDATE_REQ_REPLY_ID: u64 = 1;

mod exec;
mod reply;


//Instantiate will be used to set new requests to In Progress
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let owner = deps.api.addr_validate(&msg.owner)?;
    
    let config = Config {
        company_contract: Addr::unchecked(""),
        emp_contract: Addr::unchecked(""),
    };

    OWNER.save(deps.storage, &owner)?;
    CONFIG.save(deps.storage, &config)?;
    COMPANIES.save(deps.storage, "init".to_string(), &msg.company_info)?;
    EMPLOYEES.save(deps.storage, "init".to_string(), &msg.user_info)?;

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
        UserNewRequest{
            user_request,
        } => exec::usernewrequest(deps, info, env, user_request),
        CompanyUpdateRequest{
            update_request,
        } => exec::companyupdaterequest(deps,info, update_request),
    }
}

pub fn reply(deps: DepsMut,env:Env,reply: Reply) -> Result<Response, ContractError>{
    match reply.id{
        NEW_REQ_REPLY_ID => reply::new_request(deps, env),
        USER_UPDATE_REQ_REPLY_ID => reply::update_request(deps, env),
        id => Err(ContractError::UnrecognizedReplyID(id))
    }
}

// pub mod query{
//     use cosmwasm_std::{Deps, StdResult};

//     use crate::{msg::{EmpQueryResp, UserQueryResp}, state::{EMP_REQUESTS, USER_REQUEST}};


//     pub fn get_user(deps: Deps, request_id: String) -> StdResult<UserQueryResp>{
//         let value = USER_REQUEST.load(deps.storage, request_id)?;
//         Ok(UserQueryResp{value})
//     }
//     pub fn get_emp_requests(deps: Deps, company_name: String) -> StdResult<EmpQueryResp>{
//         let requests = EMP_REQUESTS.load(deps.storage, company_name)?;
//         Ok(EmpQueryResp{requests})
//     }
// }