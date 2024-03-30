use anyhow::Result as AnyResult;
use cosmwasm_std::{from_binary, Addr, Coin, Decimal};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::parse_execute_response_data;

use crate::contract::{execute, instantiate, query, reply};
use crate::msg::{ExecuteMsg, InstantiateMsg, ProposeMemberData, UserRequest, UpdateRequest};


#[derive(Clone, Copy, Debug)]
pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        CodeId(app.store_code(Box::new(contract)))
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: &str,
        owner: &str,
        user_info: &UserInfo,
        user_requests: &UserRequest,
        controller_contract: &str,
        is_valid: bool,
    ) -> AnyResult<Contract> {
        Contract::instantiate(
            app,
            self,
            sender,
            owner,
            user_info,
            user_requests,
            controller_contract,
            is_valid,
        )
    }
}

impl From<CodeId> for u64 {
    fn from(value: CodeId) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl Contract {
    pub fn from_addr(addr: Addr) -> Self {
        Self(addr)
    }

    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        owner: &str,
        user_info: UserInfo,
        user_requests: UserRequest,
        controller_contract: &str,
        is_valid: bool,
        
    ) -> AnyResult<Self> {
        let msg = InstantiateMsg {
            owner: owner.to_owned(),
            user_info,
            user_requests,
            controller_contract: controller_contract.to_owned(),
            is_valid,
        };

        app.instantiate_contract(code_id.0, Addr::unchecked(sender), &msg, &[], label, None)
            .map(Self)
    }

    #[track_caller]
    pub fn new_request(&self, app: &mut App, sender: &str, user_request: impl Into<&UserRequest>) -> AnyResult<()> {

        let msg = ExecuteMsg::NewRequest{user_request};

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, {})?;
        Ok(())
    }
    #[track_caller]
    pub fn update_request(&self, app: &mut App, sender: &str, update_request: impl Into<&UpdateRequest>) -> AnyResult<()> {

        let msg = ExecuteMsg::UpdateRequest{update_request};

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, {})?;
        Ok(())
    }

}