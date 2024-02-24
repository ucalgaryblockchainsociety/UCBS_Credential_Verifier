use cosmwasm_std::{Addr, DepsMut, Env, Response};

use crate::error::ContractError;
use crate::state::{Request,REQUESTS,Employees,EMPLOYEES};

// Receive request from controller
pub fn receive_request(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    // let comp_config: CompanyConfig = COMPANYCONFIG.load(deps.storage)?; 

    let request = REQUESTS.may_load(deps.storage, &user)?;
    
    if request.is_some() {
        let request = request.unwrap();

        if !request.req_status {
            return Err(ContractError::ExistingRequest {});
        }
    }

    let request: Request = Request {
        user_id: user.clone(),
        req_status: false,
        verdict: false,
        time: env.block.time.seconds(),
    };

    REQUESTS.save(deps.storage, &user, &request)?;

    Ok(Response::default().add_attribute("action", "request"))
}

// This will be fully once I figure out the NFT
pub fn validate_request(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    // let comp_config: CompanyConfig = COMPANYCONFIG.load(deps.storage)?;

    let request = REQUESTS.may_load(deps.storage, &user)?;
    if request.is_none() {
        return Err(ContractError::NoExistingRequest {});
    }

    REQUESTS.remove(deps.storage, &user);

    let employee: Employees = Employees {
        employee_account: user.clone(),
        user_info: "".to_string(),
        time: env.block.time.seconds(),
    };

    EMPLOYEES.save(deps.storage, &user, &employee)?;

    Ok(Response::new()
        .add_attribute("action", "validate"))
}