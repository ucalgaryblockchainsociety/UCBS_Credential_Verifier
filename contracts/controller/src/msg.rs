use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;
use is_empty::IsEmpty;

#[cw_serde]
pub struct UserInfo{
    pub request_id: String,
    pub employee_id: String,
    pub company: String,
    pub department: String,
    pub supervisor: String,
    pub req_status: String,
    pub contract_id: u64,
}

#[cw_serde]
pub struct CompInfo{
    pub company_id: String,
    pub company_name: String,
    pub contract_id: u64,
    pub status: String, //Whether the company has been verified or not. Can be in progress, failed or verified
}

#[cw_serde]
#[derive(Eq,IsEmpty)]
pub struct UserRequest{
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub employee_id: Option<String>,
    pub employee_name:Option<String>,
    pub company: Option<String>,
    pub verdict: Option<bool>, //true/false -- yes, employee/no, not emplpoyee
    pub department: Option<String>,
    pub supervisor: Option<String>,
    pub req_status: Option<String>, // new, complete, cancelled
    pub time: Option<u64>
}

#[cw_serde]
#[derive(Eq,IsEmpty)]
pub struct UpdateRequest{
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub verdict: Option<bool>,
    pub req_status: Option<String>,
}


#[cw_serde]
pub struct InstantiateMsg {
    pub owner:String,
    pub user_info: UserInfo,
    pub company_info: CompInfo
}

#[cw_serde]
pub enum ExecuteMsg{

    UserNewRequest{
        user_request: UserRequest,
    },
    CompanyUpdateRequest{
        update_request: UpdateRequest,
    },
    
}


#[cw_serde]
pub struct UserQueryResp{

    pub value: UserInfo, //Query response for UserRequest should give the Users Information which is a struct called UserInfo

}

#[cw_serde]
pub struct CompQueryResp{

    pub companies: CompInfo,//Query response for EmpRequests should give the list of all requests for to a specific company

}
#[cw_serde]
pub struct ReqQueryResp{

    pub user_request: UserRequest,//Query response for EmpRequests should give the list of all requests for to a specific company

}

#[cw_serde]
pub struct OwnerResp{
    pub value: Addr, //Query response for UserRequest should give the Users Information which is a struct called UserInfo
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg{

    #[returns(OwnerResp)]
    Owner{},

    #[returns (CompQueryResp)]
    QueryCompany{company_id:String},

    #[returns (UserQueryResp)]
    QueryEmployee{employee_id: String},
    
    #[returns (ReqQueryResp)]
    QueryRequest{request_id: String},
}
