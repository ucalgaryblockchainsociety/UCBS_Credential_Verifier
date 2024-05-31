use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unrecognized reply id: {0}")]
    UnrecognizedReplyID(u64),

    #[error("User request with this id is not found")]
    NoExistingRequest {},

    #[error("User not found")]
    NoExistingUser {},
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> StdError {
        StdError::generic_err(err.to_string())
    }
}
