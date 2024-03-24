use cosmwasm_std::{Addr, Deps, StdError, StdResult};

use crate::msg::{CompanyResponse, RequestResponse,EmployeeResponse};
use crate::state::{CompanyConfig,COMPANYCONFIG,REQUESTS,EMPLOYEES};

pub fn query_company_config(deps: Deps) -> StdResult<CompanyResponse> {
    let comp_config: CompanyConfig = COMPANYCONFIG.load(deps.storage)?;

    Ok(CompanyResponse{
        company_id: comp_config.company_id.to_string(),
        company_name: comp_config.company_name.to_string(),
        tax_document: comp_config.tax_document,
    })
}

pub fn query_request(deps: Deps, request: Addr) -> StdResult<RequestResponse> {
    let request = REQUESTS.may_load(deps.storage, &request)?;

    if request.is_none() {
        return Err(StdError::generic_err("No request found"));
    }

    let request = request.unwrap();

    Ok(RequestResponse {
        user_id: request.user_id.to_string(),
        req_status: request.req_status,
        verdict: request.verdict,
        time: request.time
    })
}

pub fn query_employee(deps: Deps, employee: Addr) -> StdResult<EmployeeResponse> {
    let employee = EMPLOYEES.may_load(deps.storage, &employee)?;

    if employee.is_none() {
        return Err(StdError::generic_err("Employee Not Found"));
    }

    let employee = employee.unwrap();

    Ok(EmployeeResponse {
        employee_account: employee.employee_account.to_string(),
        user_info: employee.user_info,
        time: employee.time
    })
}