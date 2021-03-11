
//! Autogenerated weights for verifier_lightclient
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-09, STEPS: [], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: Some("spec.json"), DB CACHE: 128

// Executed Command:
// ./target/release/artemis
// benchmark
// --chain
// spec.json
// --pallet
// verifier_lightclient
// --extrinsic
// *
// --repeat
// 20
// --output
// runtime/src/weights/verifier_lightclient_weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for verifier_lightclient.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> verifier_lightclient::WeightInfo for WeightInfo<T> {
	fn import_header_new_finalized_with_max_prune() -> Weight {
		(1_110_279_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(21 as Weight))
	}
	fn import_header_not_new_finalized_with_max_prune() -> Weight {
		(1_087_226_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(16 as Weight))
			.saturating_add(T::DbWeight::get().writes(20 as Weight))
	}
	fn import_header_new_finalized_with_single_prune() -> Weight {
		(1_043_057_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn import_header_not_new_finalized_with_single_prune() -> Weight {
		(1_036_348_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}