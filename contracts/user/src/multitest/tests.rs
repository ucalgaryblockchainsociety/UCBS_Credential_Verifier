use std::collections::HashMap;

// use crate::multitest::CodeId as MembershipId;
// use cosmwasm_std::Decimal;
use cw_multi_test::App;
use crate::multitest::{CodeIdUserRequest, Contractwithrequest as UserContract};
use crate::msg::{UserInfo, UserRequest};

#[test]
fn add_request() {
    let mut app = App::default();

    let denom = "star";

    let owner = String::from("owner");
    let user_id = String::from("sender1");
    let user_name = String::from("sender_name");
    let user_address = String::from("some_address");

    let request_id = String::from("req_id");
    let employee_id = String::from("emp_id");
    let company = String::from("comp");
    let department = String::from("dept");
    let supervisor = String::from("sup");
    let req_status = String::from("true");

    // let some_users = [String::from("sender1"), String::from("sender2")];
    // let candidate = "candidate";

    let userinfo_id = CodeIdUserRequest::store_code(&mut app);
    // let membership_id = MembershipId::store_code(&mut app);

    
    let info = UserInfo {
        user_id: user_id.clone(),
        user_name: user_name,
        user_address: user_address
    };

    let req = UserRequest {
        user_id: user_id.clone(),
        request_id: request_id,
        employee_id: employee_id,
        company: company,
        department: department,
        supervisor: supervisor,
        req_status: req_status
    };


    let userinfodata = userinfo_id
        .instantiate(
            &mut app,
            &user_id,
            &owner,
            info,
            req.clone(),
            "controller_contract",
            true,
        )
        .unwrap();

    let result = userinfodata.new_request(&mut app, &user_id, req.clone());

    assert!(result.is_ok(), "Expected Ok result");
    // let mut users: HashMap<String, UserContract> = HashMap::new();

    // for useri in data.some_users {
    //     users.insert(
    //         useri.owner_addr.into_string(),
    //         UserContract::from_addr(useri.user_addr),
    //     );
    // }

    // assert_eq!(users.len(), 2);
    // assert!(
    //     membership
    //         .is_member(&app, proxies[members[0]].addr().as_str())
    //         .unwrap()
    //         .is_member
    // );
    // assert!(
    //     membership
    //         .is_member(&app, proxies[members[1]].addr().as_str())
    //         .unwrap()
    //         .is_member
    // );

    // let data = users[some_users[0]]
    //     .new_request(&mut app, some_users[0], req)
    //     .unwrap();

    // assert!(data.is_none());

    // let data = proxies[members[1]]
    //     .propose_member(&mut app, members[1], candidate)
    //     .unwrap();

    // let data = data.unwrap();

    // assert_eq!(data.owner_addr, candidate);

    // assert!(
    //     membership
    //         .is_member(&app, data.proxy_addr.as_str())
    //         .unwrap()
    //         .is_member
    // );
}