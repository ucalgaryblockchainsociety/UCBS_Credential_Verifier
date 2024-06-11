use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::Addr;
use crate::msg::{CompInfo, UserInfo, UserRequest};




pub const OWNER: Item<Addr> = Item::new("owner");
pub const COMPANIES: Map<String, CompInfo> = Map::new("companies"); // Stores the Info of all companies in our system. Key(company_id), Value(CompInfo).
pub const EMPLOYEES: Map<String, UserInfo> = Map::new("users"); // Stores the Info of all employees in our system. Key(emp_id), Value(UserInfo)
pub const REQUESTS: Map<String, UserRequest> = Map::new("user_requests"); // Stores the Requests that go from employees to company and vice-versa in our system. Key(req_id), Value(UserRequest)