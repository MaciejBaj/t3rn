//! Autogenerated weights for pallet_contracts_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-02, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/circuit
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_contracts-registry
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --raw
// --template=../benchmarking/frame-weight-template.hbs
// --output
// .

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_contracts_registry.
pub trait WeightInfo {
    fn add_new_contract() -> Weight;
    fn purge() -> Weight;
    fn fetch_contracts() -> Weight;
}

/// Weights for pallet_contracts_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_new_contract() -> Weight {
        (99_670_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn purge() -> Weight {
        (62_985_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn fetch_contracts() -> Weight {
        (96_145_000 as Weight).saturating_add(T::DbWeight::get().reads(4 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn add_new_contract() -> Weight {
        (99_670_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn purge() -> Weight {
        (62_985_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn fetch_contracts() -> Weight {
        (96_145_000 as Weight).saturating_add(RocksDbWeight::get().reads(4 as Weight))
    }
}
