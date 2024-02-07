use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::{msg::UserInfo, state::{EMP_REQUESTS, USER_REQUEST}};


//Instantiate will be used to set new requests to In Progress

pub fn instantiate(
    deps: DepsMut,
    request_id: String, 
    user_info: UserInfo,
    company_name: String,
    emp_requests: Vec<String>,
) -> StdResult<Response> {
    USER_REQUEST.save(deps.storage, request_id, &user_info)?;
    EMP_REQUESTS.save(deps.storage, company_name, &emp_requests)?;//Stores all the request IDs that a certain company gets
    Ok(Response::new())
}

pub mod query{
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::QueryResp, state::USER_REQUEST};


    pub fn getUser(deps: Deps, request_id: String) -> StdResult<QueryResp>{
        let value = USER_REQUEST.load(deps.storage, request_id)?;
        Ok(QueryResp{value})
    }
}


pub mod exec{
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
    use crate::{msg::UserInfo, state::USER_REQUEST};


    pub fn newrequest(deps: DepsMut, info:MessageInfo, request_id: String, employee_id:String,company:String, department:String, supervisor:String, req_status:String) ->StdResult<Response>{
        
        let userdata = UserInfo{
            request_id: request_id,
            employee_id: employee_id,
            company: company,
            department: department,
            supervisor: supervisor,
            req_status: req_status,
        };
        USER_REQUEST.save(deps.storage, userdata.request_id.clone(), &userdata)?;

        let resp = Response::new()
            .add_attribute("action", "newrequest")
            .add_attribute("Sender", info.sender.as_str());

        Ok(resp)
    }

    pub fn updaterequest(deps: DepsMut, info:MessageInfo, req_id: String, req_status:String) ->StdResult<Response>{

        let mut ureq = USER_REQUEST.load(deps.storage, req_id.clone()).unwrap();
        
        ureq.req_status = req_status;

        USER_REQUEST.save(deps.storage, req_id, &ureq);

        let resp = Response::new()
            .add_attribute("action", "updaterequest")
            .add_attribute("Sender", info.sender.as_str());

        Ok(resp)
    }
    
}