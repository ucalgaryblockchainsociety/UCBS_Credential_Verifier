use cosmwasm_std::{DepsMut, Env, Response};

use crate::{error::ContractError, state::{PENDING_REQUESTS, USER_REQUEST}};


pub fn new_request(deps: DepsMut, env: Env) -> Result<Response, ContractError>{

    let pending_reqs = PENDING_REQUESTS.load(deps.storage)?;
    let update_id = pending_reqs.last().unwrap();

    let mut user_request = USER_REQUEST.load(deps.storage, update_id.clone())?;

    user_request.req_status = "pending".to_string();

    USER_REQUEST.save(deps.storage, update_id.clone(), &user_request)?;

    let resp = Response::new()
        .add_attribute("action", "update_request_status");

    Ok(resp)
    //should update request status in state to pending
}