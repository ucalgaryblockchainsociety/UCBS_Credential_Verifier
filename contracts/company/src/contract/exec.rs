use cosmwasm_std::{to_json_binary, Addr, DepsMut, Env, MessageInfo, Response, WasmMsg};
use is_empty::IsEmpty;

use crate::error::ContractError;
use crate::msg::{EmployeeInfo};
use crate::state::{CompanyConfig, COMPANYCONFIG, EMPLOYEES, REQUESTS};
use controller::msg::{ExecuteMsg as ControllerExecMsg, UserRequest, UpdateRequest};


// // Receive request from controller
// pub fn receive_request(deps: DepsMut, env: Env, user: String) -> Result<Response, ContractError> {
    
//     let request = REQUESTS.may_load(deps.storage, user)?;
    
//     if request.is_some() {
//         let request = request.unwrap();

//         if request.req_status.is_none() {
//             return Err(ContractError::ExistingRequest {});
//         }
//     }
   

//     REQUESTS.save(deps.storage, &user, &request)?;

//     Ok(Response::default().add_attribute("action", "request"))
// }

// This will be fully once I figure out the NFT
pub fn process_request(deps: DepsMut, env: Env, info: MessageInfo, update_request:UpdateRequest) -> Result<Response, ContractError> {
    
    let config: CompanyConfig = COMPANYCONFIG.load(deps.storage)?;

    let request = REQUESTS.may_load(deps.storage, update_request.request_id.clone().unwrap())?.unwrap();
    if request.is_empty() {
        return Err(ContractError::NoExistingRequest {});
    }
    let update_req = UpdateRequest{
        user_id: request.user_id.clone(),
        request_id: Some(update_request.request_id.clone().unwrap()),
        verdict: Some(update_request.verdict.unwrap()),
        req_status: Some(update_request.req_status.unwrap())
    };

    let controller_msg = ControllerExecMsg::CompanyUpdateRequest{update_request:update_req};
    let controller_msg = WasmMsg::Execute { 
        contract_addr: config.controller_contract.to_string(),
        msg: to_json_binary(&controller_msg)?,
        funds: vec![], 
    };
    
    let resp: Response = Response::new()
        .add_message(controller_msg)
        .add_attribute("action", "updaterequest")
        .add_attribute("sender", info.sender.as_str()); 


    let employee: EmployeeInfo = 
    EmployeeInfo {
        emp_id: request.employee_id.unwrap(),
        emp_name: request.employee_name.unwrap(),
    };

    REQUESTS.remove(deps.storage, update_request.request_id.unwrap());
    if !update_request.verdict.unwrap(){
        //Blacklist requestor
    }
    else{
        EMPLOYEES.save(deps.storage, employee.emp_id.clone(), &employee)?;
    }

    Ok(Response::new()
        .add_attribute("action", "validate"))
}