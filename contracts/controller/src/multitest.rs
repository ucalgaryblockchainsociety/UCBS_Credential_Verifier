
use cosmwasm_std::{to_json_binary, Addr, WasmMsg};
use cw_multi_test::{App, ContractWrapper, Executor};
use anyhow::Result as AnyResult;

use crate::contract::{execute, instantiate, query};
use crate::msg::{CompInfo, ExecuteMsg, InstantiateMsg, UpdateRequest, UserInfo, UserRequest};

pub struct CodeId(u64);

impl From<CodeId> for u64{
    fn from(value: CodeId) -> Self{
        value.0
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl CodeId{
    pub fn store_code(app: &mut App) -> Self{
        let contract = ContractWrapper::new(execute,instantiate,query);
        CodeId(app.store_code(Box::new(contract)))
    }
}

impl Contract{
    pub fn addr(&self) -> &Addr{
        &self.0
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        self,
        app: &mut App,
        code_id: CodeId,
        owner:String,
        user_info: UserInfo,
        company_info: CompInfo,
        label: &str
    ) -> AnyResult<Self>{
        let msg = InstantiateMsg{
            owner:owner.clone(),
            user_info,
            company_info
        };
        app.instantiate_contract(code_id.0, Addr::unchecked(owner.clone()), &msg, &[], label, None)
        .map(Self)
    }
    #[track_caller]
    pub fn usernewrequest(&self, app: &mut App, sender: &str, user_request: impl Into<UserRequest>)-> AnyResult<()>{
        let msg = ExecuteMsg::UserNewRequest{
            user_request: user_request.into(),
        };

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;

        Ok(())
    }
    #[track_caller]
    pub fn companyupdaterequest(&self, app: &mut App, sender: &str, update_request: impl Into<UpdateRequest>)-> AnyResult<()>{
        let msg = ExecuteMsg::CompanyUpdateRequest { update_request: update_request.into() };

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;

        Ok(())
    }

}

