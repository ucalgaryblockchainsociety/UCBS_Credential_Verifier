

use cw_storage_plus::{Item, Map};
use cosmwasm_std:: Addr ;

pub const COMPANYCONFIG: Item<CompanyConfig> = Item::new("config");
pub const REQUESTS: Map<&Addr, Request> = Map::new("request");
pub const EMPLOYEES: Map<&Addr, Employees> = Map::new("employee");

pub struct CompanyConfig {
    pub company_id: Addr,
    pub company_name: String,
    pub tax_document: Vec<u8>,
    // pub all_requests: &'a mut Vec<Request>,
    // pub all_employees: &'a mut Vec<Employees>
}

pub struct Request {
    pub request_id: Addr,
    pub user_id: Addr,
    pub req_status: bool,
    pub verdict: bool,
    pub time: u64
}

pub struct Employees {
    pub employee_account: Addr,
    pub user_info: String,
    pub time: u64
    // soulbound token?
}

// #[cfg_attr(not(feature = "library"), entry_point)]