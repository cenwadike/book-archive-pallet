
//! Autogenerated weights for pallet_template
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH
//! RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `kombi.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"),
//! DB CACHE:1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_template
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template
// ./scripts/frame-weight-template.hbs
// --output
// pallets/template/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
fn archive_book() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
                impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
                        /// Storage: TemplateModule ArchiveStore (r:1 w:1)
                        /// Proof Skipped: TemplateModule ArchiveStore (max_values: None, max_size: None, mode: Measured)
                        fn archive_book() -> Weight {
                        // Proof Size summary in bytes:
                        // Measured: `6`
                        // Estimated: `3471`
                        // Minimum execution time: 4_000_000 picoseconds.
                        Weight::from_parts(5_000_000,
                        3471)
                        .saturating_add(T::DbWeight::get().reads(1_u64))
                        .saturating_add(T::DbWeight::get().writes(1_u64))
                        }
                        }

                        // For backwards compatibility and tests
                        impl WeightInfo for () {
                        /// Storage: TemplateModule ArchiveStore (r:1 w:1)
                        /// Proof Skipped: TemplateModule ArchiveStore (max_values: None, max_size: None, mode: Measured)
                        fn archive_book() -> Weight {
                        // Proof Size summary in bytes:
                        // Measured: `6`
                        // Estimated: `3471`
                        // Minimum execution time: 4_000_000 picoseconds.
                        Weight::from_parts(5_000_000,
                        3471)
                        .saturating_add(RocksDbWeight::get().reads(1_u64))
                        .saturating_add(RocksDbWeight::get().writes(1_u64))
                        }
                        }