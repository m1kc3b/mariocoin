use crate::U256;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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

    pub fn hash(&self) -> ! {
        unimplemented!();
    }
}
pub struct BlockHeader {
    /// Timestamp of the block
    pub timestamp: DateTime<Utc>,             // the time when the block was created
    /// Nonce used to mine the block
    pub nonce: u64,                 // number only used once, we increment it to mine the block.
    /// Hash of the previous block
    pub prev_block_hash: [u8; 32],  // the hash of the previous block in the chain
    /// Merkle root of the block's transactions
    pub merkle_root: [u8; 32],      // the hash of the Merkle tree root derived from all of the transactions in this block
    /// Target
    pub target: U256,               // A number, which has to be higher than the hash of this block for it to be considered valid
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>, 
        nonce: u64, 
        prev_block_hash: [u8; 32], 
        merkle_root: [u8; 32], 
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

    pub fn hash(&self) -> ! {
        unimplemented!();
    }
}
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

    pub fn hash(&self) -> ! {
        unimplemented!();
    }
    
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64],
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33],
}