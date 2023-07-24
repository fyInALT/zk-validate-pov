// Copyright (C) Alt Research Ltd. All Rights Reserved.

// This source code is licensed under the limited license found in the LICENSE file
// in the root directory of this source tree.

use alloc::string::ToString;
use alloc::{vec::Vec};

use scale_codec::Decode;
// Rollup
use altbeacon_rollup_executor::{Error, InstantiateError, ReturnError, RuntimeBlob, WasmExecutor};
use altbeacon_rollup_externalities::ValidationExternalities;

// TODO: add host function
pub type HostFunctions = ();

/// The wasm method for full validation(substrate style validation) in beacon side.
pub const BEACON_VALIDATION_METHOD: &str = "beacon_validate_block";

pub fn validate_block(code: &[u8], encoded_params: &[u8]) -> Result<Vec<u8>, Error> {
    let blob = RuntimeBlob::uncompress_if_needed(code).map_err(|e| InstantiateError::Other(e))?;

    let mut executor = WasmExecutor::<HostFunctions>::new(blob, Some(8), true)?;
    let mut ext = ValidationExternalities::new();

    match executor.call(&mut ext, BEACON_VALIDATION_METHOD, encoded_params) {
        Ok(b) => Ok(b),
        Err(e) => Err(e),
    }
}
