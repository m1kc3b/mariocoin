mod sha256;
mod types;
mod utils;
mod crypto;
mod error;

use uint::construct_uint;
use serde::{Deserialize, Serialize};

construct_uint! {
    #[derive(Deserialize, Serialize)]
    pub struct U256(4);
}

pub const INITIAL_REWARD: u64 = 50;
pub const HALVING_INTERVAL: u64 = 210;
pub const IDEAL_BLOCK_TIME: u64 = 10;
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF,
]);
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
