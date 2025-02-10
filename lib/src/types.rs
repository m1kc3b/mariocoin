use crate::U256;
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::utils::MerkleRoot;
use crate::error::{Error, Result};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { 
            utxos: HashMap::new(),
            blocks: Vec::new(),
         }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        // Check if the block is valid
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash!");
                return Err(Error::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();

            if block.header.prev_block_hash != last_block.hash() {
                println!("prev hash is wrong");
                return Err(Error::InvalidBlock);
            }

            // Check if the block's hash is less than the target
            if !block.header.hash().matches_target(block.header.target) {
                println!("does not match target");
                return Err(Error::InvalidBlock)
            }

            // Check if the Merkle root is correct
            let calculated_merkle_root = MerkleRoot::calculate(&block.transactions);
            if calculated_merkle_root != block.header.merkle_root {
                println!("Invalid Merkle root");
                return Err(Error::InvalidMerkleRoot);
            }

            // Check if the block's timestamp is after the last block's timestamp
            if block.header.timestamp <= last_block.header.timestamp {
                return Err(Error::InvalidBlock);
            }

            // Verify all transactions in the block
            // block.verify_transactions(self.block_height(), &self.utxos)?;
        }

        self.blocks.push(block);
        Ok(())
    }

    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }

                for output in transaction.outputs.iter() {
                    self.utxos.insert(transaction.hash(), output.clone());
                }
            }
        }
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