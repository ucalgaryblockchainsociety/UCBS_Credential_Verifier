use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Request doesn't exist")]
    NoExistingRequest {},

    #[error("Request has already been made")]
    ExistingRequest {},

    #[error("Employee Not Found")]
    NoExistingEmployee {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
