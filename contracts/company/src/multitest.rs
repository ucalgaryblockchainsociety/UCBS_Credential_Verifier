use anyhow::Result as AnyResult;
use cosmwasm_std::{from_json, to_json_binary, Addr, WasmMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, query};
use crate::msg::{InstantiateCompanyMsg};
use cw_utils::{parse_execute_response_data, parse_instantiate_response_data};


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

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        company_id: &str,
        company_name: &str,
        tax_document: Vec<u8>,
        controller_contract: &str,
        label: &str
    ) -> AnyResult<Contract>{
        Contract::instantiate(
            app,
            self,
            sender,
            company_id,
            company_name,
            tax_document,
            controller_contract,
            label,
        )
    }

}

impl Contract{
    pub fn addr(&self)-> &Addr{
        &self.0
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        company_id: &str,
        company_name: &str,
        tax_document: Vec<u8>,
        controller_contract: &str,
        label: &str,
    ) -> AnyResult<Self>{
        let msg = InstantiateCompanyMsg{
            company_id:company_id.to_owned(),
            company_name:company_name.to_owned(),
            tax_document,
            controller_contract: controller_contract.into(),
        };
        let msg = WasmMsg::Instantiate{
            admin: None,
            code_id: code_id.0,
            msg: to_json_binary(&msg).unwrap(),
            funds: vec![],
            label:label.into() ,
        };

        let res = app.execute(Addr::unchecked(sender), msg.into())?;
        let data = parse_instantiate_response_data(res.data.unwrap_or_default().as_slice())?;

        let contract = Self(Addr::unchecked(data.contract_address));
        let data = from_json(&data.data.unwrap_or_default())?;

        Ok((contract,data))

    }



}