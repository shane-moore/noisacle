use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Proxy address is not valid")]
    InvalidProxyAddress,

    #[error("Threshold cannot be larger than number of nodes")]
    InvalidThreshold,

    #[error("Node address is not valid")]
    InvalidNodeAddress,

    #[error("Round already present")]
    JobIdAlreadyPresent,

    #[error("Unauthorized Receive execution")]
    UnauthorizedReceive,

    #[error("Unauthorized Oracle update execution")]
    UnauthorizedUpdate,

    #[error("Already submitted value for this round")]
    AlreadySubmitted,

    #[error("Received invalid randomness")]
    InvalidRandomness,

    #[error("Next round not yet started")]
    NotNextRound,
}
