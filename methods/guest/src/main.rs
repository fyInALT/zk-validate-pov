// Copyright (C) Alt Research Ltd. All Rights Reserved.

// This source code is licensed under the limited license found in the LICENSE file
// in the root directory of this source tree.

#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

mod executor;

extern crate alloc;

use alloc::{format, vec::Vec};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    // Decode the verifying key, message, and signature from the inputs.
    let params: Vec<Vec<u8>> = env::read();

    // if use the env to pass code from params, it will cost too much times
    let code = include_bytes!("../../../pov/pov_code");

    let _result = params
        .into_iter()
        .map(|param| executor::validate_block(code, &param).expect("validate_block failed"))
        .collect::<Vec<_>>();
}
