// Mock runtime

use crate as verifier;
use sp_core::H256;
use frame_support::{construct_runtime, parameter_types, traits::{Everything}};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, IdentifyAccount, Verify}, testing::Header, MultiSignature
};
use frame_system as system;
use sp_keyring::AccountKeyring as Keyring;
use super::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Verifier: verifier::{Pallet, Storage, Call, Event, Config<T>},
	}
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for MockRuntime {
	type BaseCallFilter = Everything;
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
	type PalletInfo = PalletInfo;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

impl Config for MockRuntime {
	type Event = Event;
}

pub fn new_tester() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

	verifier::GenesisConfig::<MockRuntime> {
		key: Keyring::Ferdie.into()
	}.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
