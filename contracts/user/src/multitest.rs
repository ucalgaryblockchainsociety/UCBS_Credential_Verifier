use anyhow::Result as AnyResult;
use cosmwasm_std::{from_binary, Addr, Coin, coin,  Decimal};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::parse_execute_response_data;

use crate::contract::{execute, instantiate, query, reply};
use crate::msg::{ExecuteMsg, InstantiateMsg, UserRequest, UpdateRequest, UserInfo}; // ProposeMemberData


#[derive(Clone, Copy, Debug)]
pub struct CodeIdUserInfo(u64);
#[derive(Clone, Copy, Debug)]
pub struct CodeIdUserRequest(u64);

// #[test]
#[cfg(test)]
impl CodeIdUserInfo {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query::get_user_info).with_reply(reply); // query
        CodeIdUserInfo(app.store_code(Box::new(contract)))
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

// #[test]
#[cfg(test)]
impl CodeIdUserRequest {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query::get_user_requests).with_reply(reply); // query
        CodeIdUserRequest(app.store_code(Box::new(contract)))
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

impl From<CodeIdUserInfo> for u64 {
    fn from(value: CodeIdUserInfo) -> Self {
        value.0
    }
}

impl From<CodeIdUserRequest> for u64 {
    fn from(value: CodeIdUserRequest) -> Self {
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

        app.instantiate_contract(
            code_id.0, 
            Addr::unchecked(sender), 
            &msg, 
            &[], 
            "Contract", // label
            None)
            .map(Self)
    }

    #[track_caller]
    // pub fn new_request(&self, app: &mut App, sender: &str, user_request: impl Into<&UserRequest>) -> AnyResult<()> {
    pub fn new_request<'a, T: Into<UserRequest>>(&self, app: &mut App, sender: &str, user_request: T) -> AnyResult<()> {
        let user_request: UserRequest = user_request.into();

        let msg = ExecuteMsg::NewRequest{user_request};

        let tokens = vec![
            coin(123, "utest")
        ];

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &tokens)?;
        Ok(())
    }
    #[track_caller]
    pub fn update_request<'a, T: Into<UpdateRequest>>(&self, app: &mut App, sender: &str, update_request: T) -> AnyResult<()> {
        let update_request: UpdateRequest = update_request.into();

        let msg = ExecuteMsg::UpdateRequest{update_request};

        let tokens = vec![
            coin(123, "utest")
        ];

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &tokens)?;
        Ok(())
    }

}