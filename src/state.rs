use cw_storage_plus::{Map, Item};
use cosmwasm_std::Addr;
use crate::msg::UserInfo;


//users request, maps requestID to request status
pub const USER_REQUEST: Map<String, UserInfo> = Map::new("requests");
pub const EMP_REQUESTS: Map<String, Vec<String>> = Map::new("employersrequests");
pub const OWNER: Item<Addr> = Item::new("owner");
