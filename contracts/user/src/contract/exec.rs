use cosmwasm_std::{ensure, to_json_binary, DepsMut,Env, MessageInfo, Response, SubMsg, WasmMsg};
use crate::{contract::NEW_REQ_REPLY_ID, error::ContractError, msg::{ControllerExecMsg, UpdateRequest, UserRequest}, state::{Config, CONFIG, OWNER, PENDING_REQUESTS, USER_REQUEST}};



pub fn newrequest(deps: DepsMut, _env:Env, info:MessageInfo, user_request: UserRequest) ->Result<Response, ContractError>{
        
        
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    
    let config = CONFIG.load(deps.storage)?;
    let userdata = UserRequest{
        user_id: user_request.user_id,//user_contract_id
        request_id: user_request.request_id.clone(),
        employee_id: user_request.employee_id,
        company: user_request.company,
        department: user_request.department,
        supervisor: user_request.supervisor,
        req_status: user_request.req_status,
    };

    USER_REQUEST.save(deps.storage, userdata.request_id.clone(), &userdata)?;//update the list of requests for the particultar user, using maybe .update function
    
    let mut pending_reqs = PENDING_REQUESTS.load(deps.storage)?;
    pending_reqs.push(user_request.request_id.clone());
    PENDING_REQUESTS.save(deps.storage, &pending_reqs)?;


    let controller_newreq_msg = ControllerExecMsg::NewRequest{user_request: userdata};
    let controller_newreq_msg = WasmMsg:: Execute{
        contract_addr: config.controller_contract.into_string(),
        msg: to_json_binary(&controller_newreq_msg)?,
        funds: vec![],
    };

    let controller_newreq_msg = SubMsg::reply_on_success(controller_newreq_msg, NEW_REQ_REPLY_ID);
    
    let resp = Response::new()
        .add_submessage(controller_newreq_msg)
        .add_attribute("action", "newrequest")
        .add_attribute("Sender", info.sender.as_str());

    Ok(resp)
}


pub fn updaterequest(deps: DepsMut, env:Env, info:MessageInfo, update_request: UpdateRequest) ->Result<Response, ContractError>{
    
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    let init_request = USER_REQUEST.load(deps.storage, update_request.request_id.clone())?;

    let updated_request = UserRequest{
        user_id: init_request.user_id,
        request_id: init_request.request_id,
        employee_id: init_request.employee_id,
        company: init_request.company,
        department: init_request.department,
        supervisor: init_request.supervisor,
        req_status: update_request.req_status,
    };

    USER_REQUEST.save(deps.storage, update_request.request_id.clone(), &updated_request)?;

    let resp = Response::new()
        .add_attribute("action", "updaterequest");

    Ok(resp)
    //load user request from state
    //update the request status in state
    //no need for reply on success(maybe reply the client?)
}