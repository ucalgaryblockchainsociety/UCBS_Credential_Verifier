use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use is_empty::IsEmpty;
use::controller::msg::{UserRequest,UpdateRequest};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq,JsonSchema)]
pub struct EmployeeInfo {
    pub emp_id: String, //equal to employee_id in UserRequest in User contract
    pub emp_name: String,
    // soulbound token?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateCompanyMsg {
    pub company_id: String,
    pub company_name: String,
    pub tax_document: Vec<u8>,
    pub controller_contract: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequestVerify {
    // Initiate{},
    Verify{
        update_request: UpdateRequest,
    }
    // Store the soulbound nft here?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
// #[query_responses(nested)]
#[serde(rename_all = "snake_case")]
pub enum QueryCompanyMsg {
    #[returns(CompanyResponse)]
    CompanyConfigQuery{},
    #[returns(RequestResponse)]
    RequestQuery{request_id: String},
    #[returns(EmployeeResponse)]
    EmployeeQuery{employee_account: String}
    
    // MsgCompany(CompanyInfo),
    // MsgRequest(RequestInfo),
    // MsgEmployee(EmployeeInfo),
}

//Replies
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompanyResponse {
    pub company_id: String,
    pub company_name: String,
    pub tax_document: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RequestResponse {
    pub req_info: UserRequest
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EmployeeResponse {
    pub employee_info:EmployeeInfo,
    // Give Employee list
}