use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

use crate::msg::UserInfo;


//users request, maps requestID to request status
pub const USER_REQUEST: Map<String, UserInfo> = Map::new("requests");
pub const EMP_REQUESTS: Map<String, Vec<String>> = Map::new("employersrequests");

