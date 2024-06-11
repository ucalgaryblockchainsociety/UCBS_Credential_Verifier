use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdResult};
use crate::{error::ContractError, msg::{CompInfo, ExecuteMsg, InstantiateMsg, QueryMsg, UserInfo, UserRequest}, query::{get_company, get_employee, get_owner, get_request}, state::{COMPANIES, EMPLOYEES, OWNER, REQUESTS}};


const COMP_NEW_REQ_REPLY_ID: u64 = 1;
const USER_UPDATE_REQ_REPLY_ID: u64 = 1;

mod exec;
mod reply;

//Instantiate will be used to set new requests to In Progress
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let owner = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner)?;
    COMPANIES.save(deps.storage, "init".to_string(), &msg.company_info)?;
    EMPLOYEES.save(deps.storage, "init".to_string(), &msg.user_info)?;
    REQUESTS.save(deps.storage, "init_req".to_string(),None.unwrap())?;

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


pub fn query(deps: Deps, env: Env, msg: QueryMsg)->StdResult<Binary>{
    match msg {
        QueryMsg::Owner {} =>  to_json_binary(&get_owner(deps)?),
        QueryMsg:: QueryCompany{company_id} => to_json_binary(&get_company(deps,company_id)?),
        // EmployeeInfo
        QueryMsg::QueryEmployee{employee_id} => to_json_binary(&get_employee(deps,employee_id)?),
        // {request_id} RequestInfo
        QueryMsg::QueryRequest {request_id} => to_json_binary(&get_request(deps, request_id)?),
    }
}
pub fn reply(deps: DepsMut,env:Env,reply: Reply) -> Result<Response, ContractError>{
    match reply.id{
        NEW_REQ_REPLY_ID => reply::new_request(deps, env),
        USER_UPDATE_REQ_REPLY_ID => reply::update_request(deps, env),
        id => Err(ContractError::UnrecognizedReplyID(id))
    }
}