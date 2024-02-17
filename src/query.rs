use cosmwasm_std::{Addr, Deps, StdError, StdResult};

use crate::msg::{CompanyResponse, RequestResponse,EmployeeResponse};
use crate::state::{CompanyConfig,COMPANYCONFIG,Request,REQUESTS,Employees,EMPLOYEES};

pub fn query_company_config(deps: Deps) -> StdResult<CompanyResponse> {
    let comp_config = CompanyConfig = COMPANYCONFIG.load(deps.storage)?;

    Ok(CompanyResponse{
        company_name: comp_config.company_name.to_string(),
        tax_document: comp_config.tax_document,
    })
}

pub fn query_request(deps: Deps, request: Addr) -> StdResult<CompanyResponse> {
    let request: Option<Request> = Request = REQUESTS.may_load(deps.storage, &request)?;

    if request.is_none() {
        return Err(StdError::generic_err("No request found"));
    }

    let request = request.unwrap();

    Ok(RequestResponse {
        request_id: request.request_id,
        user_id: request.user_id,
        employee_info: request.employee_info,
        req_status: request.req_status,
        verdict: request.verdict,
        time: request.time
    })
}