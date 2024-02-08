use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

#[cw_serde]
pub struct UserInfo{
    pub request_id: String,
    pub employee_id: String,
    pub company: String,
    pub department: String,
    pub supervisor: String,
    pub req_status: String,

}

#[cw_serde]
pub struct InstantiateMsg {
    pub request_id: String,
    pub user_info: UserInfo,
    pub company_name: String,
    pub emp_requests: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg{
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

#[cw_serde]
pub struct QueryResp{
    pub value: UserInfo,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg{

    #[returns (QueryResp)]
    Request{request_id:String},
}
