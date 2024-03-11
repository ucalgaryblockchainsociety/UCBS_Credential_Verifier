use cosmwasm_schema::QueryResponses;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// #[cw_serde]
// pub struct InstantiateMsg {}

// #[cw_serde]
// pub struct InstantiateRequestMsg {
//     pub user_id: String,
// }

// #[cw_serde]
// pub enum ExecuteMsg {}

// #[derive(Serialize, Deserialize)]
// pub enum CompanyVerify {
//     CompanyConfig{tax_document: Vec<u8>}
//     // Store the soulbound nft here?
// }


// #[derive(Serialize, Deserialize)]
// pub enum EmployeeCreate {
//     Employeee{employee_account: String}
//     // Store the soulbound nft here?
// }

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
// #[query_responses(nested)]
#[serde(rename_all = "snake_case")]
pub enum QueryCompanyMsg {
    #[returns(CompanyResponse)]
    CompanyConfig{},
    #[returns(RequestResponse)]
    Request{request_id: String},
    #[returns(EmployeeResponse)]
    Employees{employee_account: String}
    // MsgCompany(CompanyInfo),
    // MsgRequest(RequestInfo),
    // MsgEmployee(EmployeeInfo),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequestVerify {
    Initiate{},
    Verify{}
    // Store the soulbound nft here?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateCompanyMsg {
    pub company_name: String,
    pub tax_document: Vec<u8>,
}

// #[derive(JsonSchema, QueryResponses)]
// enum CompanyInfo {
//     #[returns(CompanyResponse)]
//     CompanyConfig{},
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompanyResponse {
    pub company_id: String,
    pub company_name: String,
    pub tax_document: Vec<u8>,
    // pub all_requests: &'a mut Vec<T>,
    // pub all_employees: &'a mut Vec<T>
}

// #[derive(JsonSchema, QueryResponses)]
// enum RequestInfo {
//     #[returns(RequestResponse)]
//     Request{user_id: String},
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RequestResponse {
    // give company request lists
    pub user_id: String,
    pub req_status: bool,
    pub verdict: bool,
    pub time: u64
}

// #[derive(JsonSchema, QueryResponses)]
// enum EmployeeInfo {
//     #[returns(EmployeeResponse)]
//     Employees{employee_account: String},
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EmployeeResponse {
    pub employee_account: String,
    pub user_info: String,
    pub time: u64
    // Give Employee list
}
