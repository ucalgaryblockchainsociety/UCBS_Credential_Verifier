use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct UserInfo{
    pub request_id: String,
    pub employee_id: String,
    pub company: String,
    pub department: String,
    pub supervisor: String,
    pub req_status: String,
    pub contract_id: Addr,
}

#[cw_serde]
pub struct UserRequest{
    pub user_id: String,
    pub request_id: String,
    pub employee_id: String,
    pub company: String,
    pub department: String,
    pub supervisor: String,
    pub req_status: String,
}

#[cw_serde]
pub struct UpdateRequest{
    pub user_id: String,
    pub request_id: String,
    pub req_status: String,
}


#[cw_serde]
pub struct CompInfo{
    pub company_id: String,
    pub company_name: String,
    pub contract_id: Addr,
    pub status: String, //Whether the company has been verified or not. Can be in progress, failed or verified
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner:String,
    pub company_contract: Addr,
    pub emp_contract: Addr,
    pub user_info: UserInfo,
    pub company_info: CompInfo,
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
pub enum CompanyExecMsg{
    NewRequest{
        user_request: UserRequest,
    },
}
#[cw_serde]
pub enum UserExecMsg{

    UpdateRequest { //Foreign message for executing on the User contract. Intended to send updates to verification requests from an employer to the user.
        update_request: UpdateRequest,
        //Depending on status, update user wallet with soulbound NFT, may need users wallet address
    },

}


#[cw_serde]
pub struct UserQueryResp{

    pub value: UserInfo, //Query response for UserRequest should give the Users Information which is a struct called UserInfo

}
#[cw_serde]
pub struct CompQueryResp{

    pub companies: Vec<String>,//Query response for EmpRequests should give the list of all requests for to a specific company

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg{

    #[returns (UserQueryResp)]
    UserRequest{request_id:String},

    #[returns (CompQueryResp)]
    EmpRequests{company_name: String},
}
