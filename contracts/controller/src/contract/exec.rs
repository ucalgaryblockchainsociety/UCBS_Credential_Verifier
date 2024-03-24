
use cosmwasm_std::{ensure, to_json_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};

use crate::{contract::USER_UPDATE_REQ_REPLY_ID, error::ContractError, msg::{CompanyExecMsg, UpdateRequest, UserExecMsg, UserRequest}, state::{COMPANIES, EMPLOYEES, OWNER}};

use super::COMP_NEW_REQ_REPLY_ID;


pub fn usernewrequest(deps: DepsMut, info:MessageInfo, env: Env, user_request: UserRequest )->Result<Response, ContractError>{

    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    let company_id = user_request.company.clone();

    let company_info = COMPANIES.load(deps.storage, company_id)?;

    let company_contract_id = company_info.contract_id;
    
    let company_newreq_msg = CompanyExecMsg::NewRequest{user_request: user_request.clone()};
    let company_newreq_msg = WasmMsg:: Execute{
        contract_addr: company_contract_id.to_string(),
        msg: to_json_binary(&company_newreq_msg)?,
        funds: vec![],
    };

    let company_newreq_msg = SubMsg::reply_on_success(company_newreq_msg, COMP_NEW_REQ_REPLY_ID);
    
    let resp = Response::new()
        .add_submessage(company_newreq_msg)
        .add_attribute("action", "newrequest")
        .add_attribute("Sender", user_request.user_id.clone());

    Ok(resp)


    //Check for the company in the userRequest in the COMPANIES state.
 
    //Retreive the company info, get the contract ID.
    //Execute newrequestupdate on the company contract
    //send reply to the user contract to let them know message sent to employer successfully

}

pub fn companyupdaterequest(deps: DepsMut,info:MessageInfo, update_request: UpdateRequest)->Result<Response, ContractError>{
    
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);
    
    let user_info = EMPLOYEES.load(deps.storage, update_request.user_id.clone())?;

    let user_contract_id = user_info.contract_id;

    let user_update_msg = UserExecMsg::UpdateRequest { update_request: update_request };
    let user_update_msg = WasmMsg::Execute { 
        contract_addr: user_contract_id.to_string(), 
        msg: to_json_binary(&user_update_msg)?,
        funds: vec![] 
    };


    let user_update_msg = SubMsg::reply_on_success(user_update_msg, USER_UPDATE_REQ_REPLY_ID);
    
    let resp = Response::new()
        .add_submessage(user_update_msg)
        .add_attribute("action", "requestupdate")
        .add_attribute("Sender", info.sender);

    Ok(resp)
    //receive update request from company. 

    //find the user contract the request is sent for
    //retreive user contract_id
    //send request to user contract
    //send reply to company contract to confirm transaction

}