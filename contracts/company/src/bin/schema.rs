use cosmwasm_schema::write_api;

use company::msg::{RequestVerify, InstantiateCompanyMsg, QueryCompanyMsg};

fn main() {
    write_api! {
        instantiate: InstantiateCompanyMsg,
        execute: RequestVerify,
        query: QueryCompanyMsg,
    }
}
