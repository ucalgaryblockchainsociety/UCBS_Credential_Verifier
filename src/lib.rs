pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

use contract::query;
use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::{ExecuteMsgs, InstantiateMsg, QueryMsgs};

#[entry_point]
pub fn instantiate(
    deps:DepsMut,
    _env:Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response>{
    contract::instantiate(deps, msg.request_id,msg.user_info)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsgs) -> StdResult<Binary>{
    use crate::QueryMsgs::Request;
    match msg{
        Request{request_id} => to_json_binary(&query::getUser(deps, request_id)?),
    }
}



#[entry_point]
pub fn execute(deps:DepsMut, env: Env, info:MessageInfo, msg:ExecuteMsgs) ->StdResult<Response>{

    use contract::exec;
    use msg::ExecuteMsgs::*;

    match msg{
        NewRequest{ 
            req_id, 
            employee_id,
            company, 
            department,
            supervisor,
            req_status
        } => exec::newrequest(deps, info, req_id, employee_id,company, department, supervisor, req_status),
        UpdateRequest{ req_id, req_status} => exec::updaterequest(deps, info, req_id, req_status),
    }
}

#[cfg(test)]

mod test{
    use std::default;

    use crate::msg::{QueryMsgs, QueryResp, UserInfo};

    use super::*;
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App,Contract, ContractWrapper, Executor, Router};
    fn request_contract() -> Box<dyn Contract<Empty>>{
        let contract = ContractWrapper::new(execute,instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn new_request(){
        let mut app = App::default();
        let contract_id = app.store_code(request_contract());
        let sender = Addr::unchecked("sender");
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg{request_id: "1".to_string(), user_info: UserInfo { request_id: "1".to_string(), employee_id: "444".to_string(), company: "atco".to_string(), department: "projects".to_string(), supervisor: "brett".to_string(), req_status: "verified".to_string() }},
                &[],
                "Request contract",
                None,
            ).unwrap();
        app.execute_contract(sender, contract_addr.clone(),&ExecuteMsgs::NewRequest { req_id: "2".to_string(), employee_id: "444".to_string(), company: "atco".to_string(), department: "projects".to_string(), supervisor: "brett".to_string(), req_status: "verified".to_string() }, &[]).unwrap();

        let resp: QueryResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsgs::Request { request_id: "1".to_string() })
            .unwrap();
        
            let userdata = UserInfo{
                request_id: "2".to_string(),
                employee_id: "444".to_string(),
                company: "atco".to_string(),
                department: "projects".to_string(),
                supervisor: "brett".to_string(),
                req_status: "verified".to_string(),
            };
        assert_eq!(resp, QueryResp{value:userdata});

    }
}