use std::time;

use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::Addr;
use crate::msg::{UserInfo, UserRequest};

#[cw_serde]
pub struct Config{
    pub controller_contract: Addr,
    pub is_valid: bool,
}
#[cw_serde]
pub struct RequestData {
    pub req_id: String,
}

//users request, maps requestID to request status
pub const OWNER: Item<Addr> = Item::new("owner");
pub const CONFIG: Item<Config> = Item::new("config");
pub const USER_INFO: Item<UserInfo>  = Item::new("user_info");
pub const USER_REQUEST: Map<String,UserRequest> = Map::new("user_requests");// State to store all requests for a user accompanied with the users information.<key: user_id,value: vec<userrequest>>
pub const PENDING_REQUESTS: Item<Vec<String>> = Item::new("pending_requests");//stores req_id of all pending requests for this user 