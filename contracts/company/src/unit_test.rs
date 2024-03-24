use crate::contract::{instantiate,execute,query};
use crate::msg::{InstantiateCompanyMsg,RequestVerify,QueryCompanyMsg,CompanyResponse,RequestResponse,EmployeeResponse};
use cosmwasm_std::{to_json_binary, from_json, MessageInfo, Empty, Addr, Binary};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw721::Cw721ReceiveMsg;

use std::fs;
use std::fs::File;
use std::io::Read;

#[test]
fn test_company_instantiation() {
    let mut deps = mock_dependencies();

    // Got help from: https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
    // let filename = "test_resources/test_tax_doc.pdf";

    // let mut f = File::open(&filename).expect("no file found");
    // let metadata = fs::metadata(&filename).expect("unable to read metadata");
    // let mut buffer = vec![0; metadata.len() as usize];
    // f.read(&mut buffer).expect("buffer overflow");
    let len = 10;
    let buffer = vec![0; len];

    let msg = InstantiateCompanyMsg {
        company_name: "Test_Company".to_string(),
        tax_document: buffer.clone(),
    };
    let info = mock_info("some_id", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryCompanyMsg::CompanyConfig {}).unwrap();
    let actual_config: CompanyResponse = from_json(&res).unwrap();
    let info = mock_info("some_id", &[]);
    let cloned_buf = buffer.clone();
    assert_eq!(
        actual_config,
        CompanyResponse {
            company_id: info.sender.to_string(),
            company_name: "Test_Company".to_string(),
            tax_document: cloned_buf,
        }
    );
}

#[test]
fn poke_request() {
    let mut app = App::default();
 
    let contract_id = app.store_code(company_contract());

    let buffer = vec![1, 2, 3, 4, 5];
    let tax_document = buffer.clone();
    let company_name = "Test Company".to_string();


    let contract_addr = app
    .instantiate_contract(
        contract_id,
        Addr::unchecked("sender"),
        &InstantiateCompanyMsg { company_name, tax_document},
        &[],
        "Counting contract",
        None,
    )
    .unwrap();

    let info = mock_info("requestor", &[]);
    let request_id = info.sender.to_string().clone();

    app.execute_contract(
        info.sender.clone(),
        contract_addr.clone(),
        &RequestVerify::Receive(Cw721ReceiveMsg{ 
            sender: "requestor".to_string(), 
            token_id: "123456789".to_string(), 
            msg: Binary::from("Some other info".as_bytes())
        }),
        &[],
    )
    .unwrap();

    let resp: RequestResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryCompanyMsg::Request {request_id})
        .unwrap();
 
    assert_eq!(resp, RequestResponse { 
        user_id: info.sender.to_string().clone(),
        req_status: false,
        verdict: false,
        time: mock_env().block.time.seconds()
    });
}


fn company_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
// #[test]
// fn query_values(){
//     let mut app = App::defualt();

//     let contract_id = app.store_code(company_contract());

//     // Got help from: https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
//     let mut filename = "../test_resources/test_tax_doc.pdf";

//     let mut f = File::open(&filename).expect("no file found");
//     let metadata = fs::metadata(&filename).expect("unable to read metadata");
//     let mut buffer = vec![0; metadata.len() as usize];
//     f.read(&mut buffer).expect("buffer overflow");

//     let contract_addr = app.instantiate_contract(
//         contract_id,
//         Addr::unchecked("sender"),
//         &QueryCompanyMsg::CompanyConfig{},
//         &[],
//         "Counting Contract",
//         None
//     ).unwrap();

//     let resp: CompanyResponse = app.wrap().query_wasm_smart(contract_addr, &QueryCompanyMsg::CompanyConfig{});

//     assert_eq!(resp, CompanyResponse {company_name: "Test Company", tax_document:buffer});
// }
