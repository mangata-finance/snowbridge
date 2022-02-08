//! # Verifier
//!
//! The verifier module provides functionality for message verification.
//!
//! ## Overview
//!
//! This verifier performs the following verification routines on a message:
//! - Ensuring that the message sender is trusted
//! - Ensuring that messages are not replayed
//!
//! This verifier is intended to be swapped out for an Ethereum light-client solution at some point.
//!
//! ## Interface
//!
//! The verifier implements the [`Verifier`] trait and conforms to its interface.
//!
#![allow(unused_variables)]
#![cfg_attr(not(feature = "std"), no_std)]



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::{decl_module, decl_storage, decl_event, decl_error,
	dispatch::DispatchResult};
use sp_runtime::traits::Hash;
use sp_std::prelude::*;
use artemis_core::{AppId, Message, Verifier, VerificationInput};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event {
	Dummy
}

#[pallet::config]
pub trait Config: frame_system::Config {
	type Event: From<Event> + IsType<<Self as frame_system::Config>::Event>;
}

#[pallet::storage]
#[pallet::getter(fn key)]
pub type RelayKey<T:Config> = StorageValue<_, Option<T::AccountId>, ValueQuery>;

#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
	pub key: Option<T::AccountId>
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	fn build(&self) {
		RelayKey::<T>::put(self.key.clone());
	}
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
	fn default() -> Self {
		GenesisConfig { key: None }
	}
}

/// Hashes of previously seen messages. Used to implement replay protection.
#[pallet::storage]
pub(super) type VerifiedPayloads<T: Config> = StorageMap<_, Blake2_256, T::Hash, (), ValueQuery>;


#[pallet::error]
pub enum Error<T> {
	/// Verification scheme is not supported.
	NotSupported,
	/// The message failed verification.
	Invalid
}


#[pallet::pallet]
#[pallet::generate_store(pub(super) trait Store)]
pub struct Pallet<T>(PhantomData<T>);

#[pallet::call]
impl<T: Config> Pallet<T> {
}

impl<T: Config> Pallet<T> {

	/// Verify a message
	pub fn do_verify(sender: T::AccountId, app_id: AppId, message: &Message) -> DispatchResult {
		Self::verify_sender(sender)?;

		// Hash all inputs together to produce a unique key for the message
		let (block_no, event_idx) = match message.verification {
			VerificationInput::Basic { block_number, event_index } => (block_number, event_index),
			VerificationInput::None => return Err(Error::<T>::NotSupported.into())
		};
		let key_input = (app_id, message.payload.clone(), block_no, event_idx);
		let key = T::Hashing::hash_of(&key_input);

		// Verify that the message has not been seen before (replay protection)
		if <VerifiedPayloads<T>>::contains_key(key) {
			return Err(Error::<T>::Invalid.into())
		} else {
			<VerifiedPayloads<T>>::insert(key, ());
		}

		Ok(())
	}

	// Verify that the message sender matches the relayer account
	pub fn verify_sender(sender: T::AccountId) -> DispatchResult {
		if let Some(sender) = RelayKey::<T>::get(){
			Ok(())
		}else{
			return Err(Error::<T>::Invalid.into())
		}
	}
}

impl<T: Config> Verifier<T::AccountId> for Pallet<T> {
	fn verify(sender: T::AccountId, app_id: AppId, message: &Message) -> DispatchResult {
		Self::do_verify(sender, app_id, message)?;
		Ok(())
	}
}
}
