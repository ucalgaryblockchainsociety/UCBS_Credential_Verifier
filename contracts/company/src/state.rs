use cw_storage_plus::{Item, Map};
use cosmwasm_std:: Addr ;
use serde::{Deserialize, Serialize};

use crate::msg::{EmployeeInfo};
use controller::msg::{UserRequest};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompanyConfig {
    pub company_id: String,
    pub company_name: String,
    pub tax_document: Vec<u8>,
    pub controller_contract: Addr,
}



pub const COMPANYCONFIG: Item<CompanyConfig> = Item::new("config");
pub const REQUESTS: Map<String, UserRequest> = Map::new("request"); //request_id, Request Info
pub const EMPLOYEES: Map<String, EmployeeInfo> = Map::new("employee"); // employee_id, emolyee_info
















// #[cfg_attr(not(feature = "library"), entry_point)]

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
// pub struct CompanyConfig {
//     pub company_id: Addr,
//     pub company_name: String,
//     pub tax_document: Vec<u8>,
//     // pub all_requests: &'a mut Vec<Request>,
//     // pub all_employees: &'a mut Vec<Employees>
// }