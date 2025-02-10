use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Invalid transaction")]
  InvalidTransaction,
  #[error("Invalid block")]
  InvalidBlock,
  #[error("Invalid block header")]
  InvalidBlockHeader,
  #[error("Invalid transaction imput")]
  InvalidTransactionInput,
  #[error("Invalid transaction output")]
  InvalidTransactionOutput,
  #[error("Invalid Merkle root")]
  InvalidMerkleRoot,
  #[error("Invalid hash")]
  InvalidHash,
  #[error("Invalid signature")]
  InvalidSignature,
  #[error("Invalid public key")]
  InvalidPublicKey,
  #[error("Invalid private key")]
  IvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, Error>;