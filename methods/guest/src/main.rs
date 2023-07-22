#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

extern crate alloc;

use alloc::vec::Vec;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    // Decode the verifying key, message, and signature from the inputs.
    let params: Vec<Vec<u8>> = env::read();

    // if use the env to pass code from params, it will cost too much times
    let code = include_bytes!("../../../pov/pov_code");

    // let validate_pov risc0 code contain all code datas.
    // this will let us to test the costs.
    env::commit(&code[params.len()]);
}
