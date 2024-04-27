use ethereum_consensus::Error as ConsensusError;
use reth::payload::error::PayloadBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Consensus(#[from] ConsensusError),
    #[error(transparent)]
    PayloadBuilderError(#[from] PayloadBuilderError),
}
