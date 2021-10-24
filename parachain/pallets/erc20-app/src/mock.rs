// Mock runtime

use crate as erc20App;
use frame_support::{construct_runtime, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
    MultiSignature,
};
use sp_std::convert::From;

pub use artemis_asset as asset;
use mangata_primitives::{Amount, Balance, TokenId};
use orml_tokens::MultiTokenCurrencyAdapter;
use orml_traits::parameter_type_with_key;

use super::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
        BridgedAssets: asset::{Module, Call, Storage, Config<T>, Event<T>},
		Tokens: orml_tokens::{Module, Storage, Call, Event<T>, Config<T>},
        ERC20: erc20App::{Module, Storage, Call, Event<T>},
	}
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for MockRuntime {
    type BaseCallFilter = ();
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
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: TokenId| -> Balance {
		match currency_id {
			_ => 0,
		}
	};
}

impl orml_tokens::Config for MockRuntime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = TokenId;
    type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
}

impl asset::Config for MockRuntime {
    type Event = Event;
    type Currency = MultiTokenCurrencyAdapter<MockRuntime>;
}

impl Config for MockRuntime {
    type Event = Event;
}

pub fn new_tester() -> sp_io::TestExternalities {
    let storage = system::GenesisConfig::default()
        .build_storage::<MockRuntime>()
        .unwrap();
    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
