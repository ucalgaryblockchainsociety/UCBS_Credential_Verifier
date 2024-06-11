use controller::msg::UpdateRequest;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::StdError;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateCompanyMsg, QueryCompanyMsg,RequestVerify};
use crate::state::{CompanyConfig,COMPANYCONFIG};
use crate::query::{query_company_config,query_request,query_employee};
mod exec;
mod reply;


/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosm-wasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateCompanyMsg,
) -> Result<Response, ContractError> {
    let controller_contract =
      deps.api.addr_validate(&msg.controller_contract)?;
    // unimplemented!() // Ok(Response::new())
    let config: CompanyConfig = CompanyConfig {
        company_id: msg.company_id,
        company_name: msg.company_name,
        tax_document: msg.tax_document,
        controller_contract: controller_contract,
    };

    COMPANYCONFIG.save(deps.storage, &config)?;

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
        //RequestVerify::Initiate{} => receive_request(_deps, _env,  _info.sender),
        RequestVerify::Verify{update_request} => exec::process_request(_deps, _env,_info,update_request),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryCompanyMsg) -> StdResult<Binary> {
    match _msg {
        QueryCompanyMsg::CompanyConfigQuery{} => to_json_binary(&query_company_config(_deps)?),
        // EmployeeInfo
        QueryCompanyMsg::EmployeeQuery {employee_account} => {
            match &query_employee(_deps, employee_account.clone()) {

                Ok(result) => to_json_binary(&result),
                Err(_) => {
                    to_json_binary(&query_request(_deps, employee_account.clone())?)
                },
            }
        },
        // {request_id} RequestInfo
        QueryCompanyMsg::RequestQuery {request_id} => {
            to_json_binary(&query_request(_deps, request_id)?)
        }
    }
}

#[cfg(test)]
mod tests {}
