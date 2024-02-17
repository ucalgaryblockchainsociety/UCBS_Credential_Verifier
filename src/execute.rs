use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, DepsMut, Env, Response, Uint128, WasmMsg};

use crate::error::ContractError;
use crate::state::{CompanyConfig,COMPANYCONFIG,Request,REQUESTS,Employees,EMPLOYEES};

// Here I will make a function for validating requests