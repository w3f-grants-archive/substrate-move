// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(test)]
mod tests;

#[cfg(feature = "std")]
pub mod utils;

pub mod natives;

#[cfg(feature = "std")]
pub mod doc;

#[cfg(feature = "stdlib-bytecode")]
/// Provides a precompiled bundle of move-stdlib bytecode modules.
pub fn move_stdlib_bundle() -> &'static [u8] {
    include_bytes!("../build/MoveStdlib/bundles/MoveStdlib.mvb")
}

#[cfg(feature = "stdlib-bytecode")]
/// Provides a precompiled bundle of substrate-stdlib bytecode modules.
pub fn substrate_stdlib_bundle() -> &'static [u8] {
    include_bytes!("../substrate-stdlib/build/substrate-stdlib/bundles/substrate-stdlib.mvb")
}
