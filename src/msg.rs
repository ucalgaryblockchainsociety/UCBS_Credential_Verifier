use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct InstantiateCompanyMsg {
    pub company_id: String,
    pub company_name: String,
    pub tax_document: Vec<u8>,
}

#[cw_serde]
pub struct InstantiateRequestMsg {
    pub user_id: String,
}

// #[cw_serde]
// pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize)]
pub enum CompanyVerify {
    CompanyConfig{tax_document: Vec<u8>}
    // Store the soulbound nft here?
}

#[derive(Serialize, Deserialize)]
pub enum RequestVerify {
    Request{request_id: String}
    // Store the soulbound nft here?
}

#[derive(Serialize, Deserialize)]
pub enum EmployeeCreate {
    Employeee{employee_account: String}
    // Store the soulbound nft here?
}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {}

#[cw_serde]
pub enum QueryCompanyMsg {
    CompanyConfig{},
    Request{request_id: String}
}

// #[cw_serde]
// pub enum RequestMsg {
//     Request{}
// }

#[cw_serde]
pub enum EmployeeMsg {
    Employee{}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CompanyResponse {
    pub company_name: String,
    pub tax_document: Vec<u8>,
    // pub all_requests: &'a mut Vec<T>,
    // pub all_employees: &'a mut Vec<T>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RequestResponse {
    // give company request lists
    pub request_id: String,
    pub user_id: String,
    pub employee_info: String,
    pub req_status: bool,
    pub verdict: bool,
    pub time: u64
}

#[derive(Serialize, Deserialize)]
pub struct EmployeeResponse {
    pub employee_account: String,
    pub user_info: String,
    pub time: u64
    // Give Employee list
}