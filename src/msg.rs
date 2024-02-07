use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserInfo{
    pub request_id: String,
    pub employee_id: String,
    pub company: String,
    pub department: String,
    pub supervisor: String,
    pub req_status: String,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub struct InstantiateMsg {
    pub request_id: String,
    pub user_info: UserInfo,
    pub company_name: String,
    pub emp_requests: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum ExecuteMsgs {
    NewRequest{
        req_id: String,
        employee_id: String,
        company: String,
        department: String,
        supervisor: String,
        req_status: String,
    },
    UpdateRequest{
        req_id: String,
        req_status: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub struct QueryResp{
    pub value: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum QueryMsgs {
    Request{request_id:String},
}
