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

fn main() {
    println!("Hello, world!");
}
