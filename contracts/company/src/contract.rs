use cosmwasm_std::StdError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,entry_point,to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateCompanyMsg, QueryCompanyMsg,RequestVerify};
use crate::state::{CompanyConfig,COMPANYCONFIG};
use crate::query::{query_company_config,query_request,query_employee};
use crate::execute::{receive_request,validate_request};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosm-wasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateCompanyMsg,
) -> Result<Response, ContractError> {
    // unimplemented!() // Ok(Response::new())
    let config: CompanyConfig = CompanyConfig {
        company_id: _info.sender,
        company_name: _msg.company_name,
        tax_document: _msg.tax_document
    };

    COMPANYCONFIG.save(_deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: RequestVerify,
) -> Result<Response, ContractError> {
    match _msg {
        RequestVerify::Initiate{} => receive_request(_deps, _env,  _info.sender),
        RequestVerify::Verify{} => validate_request(_deps, _env,_info.sender),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryCompanyMsg) -> StdResult<Binary> {
    // unimplemented!()
    match _msg {
        QueryCompanyMsg::CompanyConfig {} => to_json_binary(&query_company_config(_deps)?),
        // EmployeeInfo
        QueryCompanyMsg::Employees {employee_account} => {
            match &query_employee(_deps, _deps.api.addr_validate(&employee_account)?) {
                Ok(result) => to_json_binary(&result),
                Err(_) => {
                    to_json_binary(&query_request(_deps, _deps.api.addr_validate(&employee_account)?)?)
                },
            }
        },
        // {request_id} RequestInfo
        QueryCompanyMsg::Request {request_id} => {
            to_json_binary(&query_request(_deps, _deps.api.addr_validate(&request_id)?)?)
        }
    }
}

#[cfg(test)]
mod tests {}