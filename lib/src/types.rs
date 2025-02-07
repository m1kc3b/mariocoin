use crate::U256;
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::utils::MerkleRoot;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Block {
        Block { 
            header, 
            transactions, 
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    /// Timestamp of the block
    pub timestamp: DateTime<Utc>,             
    /// Nonce used to mine the block
    pub nonce: u64,                 
    /// Hash of the previous block
    pub prev_block_hash: Hash,  
    /// Merkle root of the block's transactions
    pub merkle_root: MerkleRoot,      
    /// Target
    pub target: U256,               
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>, 
        nonce: u64, 
        prev_block_hash: Hash, 
        merkle_root: MerkleRoot, 
        target: U256
    ) -> BlockHeader {
        BlockHeader { 
            timestamp, 
            nonce, 
            prev_block_hash, 
            merkle_root, 
            target, 
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Transaction {
        Transaction { 
            inputs, 
            outputs, 
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
    
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}