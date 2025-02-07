mod sha256;
mod types;
mod utils;
mod crypto;

use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

fn main() {
    println!("Hello, world!");
}
