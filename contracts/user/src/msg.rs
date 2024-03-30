use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct UserInfo{
    pub user_id: String,
    pub user_name: String,
    pub user_address: String,
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
pub struct InstantiateMsg {
    pub owner: String,
    pub user_info: UserInfo,    
    pub user_requests: UserRequest,
    pub controller_contract: String,
    pub is_valid: bool,
}

#[cw_serde]
pub enum ExecuteMsg{
    NewRequest{
        user_request: UserRequest,
    },
    UpdateRequest{
        update_request: UpdateRequest,
    },
}

#[cw_serde]
pub enum UserExecMsg{
    UpdateRequest { //Foreign message for executing on the User contract. Intended to send updates to verification requests from an employer to the user.
        req_id: String,
        req_status: String,
        //Depending on status, update user wallet with soulbound NFT, may need users wallet address
    },

}

#[cw_serde]
pub enum ControllerExecMsg{ 
    NewRequest{ //Foreign message for executing on the employer contract. Intended to send new requests to an employer
        user_request: UserRequest,
    },
    UpdateRequest{
        req_id:String,
        req_status:String,
    }
} 
  

#[cw_serde]
pub struct UserReqQueryResp{
    pub value: UserRequest, //Query response for UserRequest should give the Users Information which is a struct called UserInfo
}

#[cw_serde]
pub struct UserInfoQueryResp{
    pub value: UserInfo, //Query response for UserRequest should give the Users Information which is a struct called UserInfo
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg{

    #[returns (UserReqQueryResp)]
    UserRequests{request_id:String},

    #[returns (UserInfoQueryResp)]
    UserInfo{},

}
