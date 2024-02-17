#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,entry_point,to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CompanyVerify, InstantiateCompanyMsg, QueryCompanyMsg,RequestVerify};
use crate::state::{CompanyConfig,COMPANYCONFIG};
use crate::query::{query_company_config};

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
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryCompanyMsg) -> StdResult<Binary> {
    // unimplemented!()
    match _msg {
        QueryCompanyMsg::CompanyConfig {} => to_json_binary(&query::query_company_config(_deps?)?),
        QueryCompanyMsg::Request { address:String } => {
            to_json_binary(&query_requests(deps, deps.api.addr_validate(&address)?)?)
        }
    }
}

#[cfg(test)]
mod tests {}
