use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::Addr;
use crate::msg::{UserInfo, CompInfo};

#[cw_serde]
pub struct Config{
    pub company_contract: Addr,
    pub emp_contract: Addr,
}


pub const OWNER: Item<Addr> = Item::new("owner");
pub const CONFIG: Item<Config> = Item::new("config"); // configuration
pub const COMPANIES: Map<String, CompInfo> = Map::new("companies"); // Stores the Info of all companies in our system. Key(company_id), Value(CompInfo).
pub const EMPLOYEES: Map<String, UserInfo> = Map::new("users"); // Stores the Info of all employees in our system. Key(emp_id), Value(UserInfo)