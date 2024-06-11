

use cosmwasm_std::{Deps, StdResult};

use crate::msg::{CompQueryResp, OwnerResp, ReqQueryResp, UserQueryResp};
use crate::state::{COMPANIES, EMPLOYEES, OWNER, REQUESTS};


pub fn get_owner(deps: Deps) -> StdResult<OwnerResp>{
    let value = OWNER.load(deps.storage)?;
    Ok(OwnerResp{value})
}

pub fn get_company(deps: Deps, company_id: String) -> StdResult<CompQueryResp>{
    let value = COMPANIES.load(deps.storage, company_id)?;
    Ok(CompQueryResp{companies:value})
}
pub fn get_employee(deps: Deps, employee_id: String) -> StdResult<UserQueryResp>{
    let value = EMPLOYEES.load(deps.storage, employee_id)?;
    Ok(UserQueryResp{value})
}
pub fn get_request(deps: Deps, request_id: String) -> StdResult<ReqQueryResp>{
    let request = REQUESTS.load(deps.storage, request_id)?;
    Ok(ReqQueryResp{user_request:request})
}