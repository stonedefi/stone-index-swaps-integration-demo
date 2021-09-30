use crate as pallet_stone_swaps_bridge;
use frame_support::{construct_runtime, parameter_types};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

pub const ACCOUNT_ID: u64 = 6798534;
pub const INDEX_ID: u64 = 1;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Assets: pallet_assets::{Module, Call, Event<T>},
        StoneIndex: pallet_stone_index::{Module, Call, Storage, Event<T>},
        Fungible: pallet_fungible::{Module, Call, Storage, Event<T>},
        Swaps: pallet_swaps::{Module, Call, Storage, Event<T>},      
        Balances: pallet_balances::{Module, Call, Storage, Event<T>},
        StoneSwapBridge: pallet_stone_swaps_bridge::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const ExistentialDeposit: u64 = 1;
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    }

impl pallet_assets::Config for TestRuntime {
    type Event = ();
    type Balance = u64;
    type AssetId = u64;
}

impl pallet_stone_index::Config for TestRuntime {
    type Event = ();
    type IndexId = u64;
}

impl pallet_balances::Config for TestRuntime {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<TestRuntime>;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl pallet_swaps::Config for TestRuntime {
    type Event = ();
    type SwapId = u64;
    type Currency = pallet_balances::Module<TestRuntime>;
}

impl pallet_fungible::Config for TestRuntime {
    type Event = ();
    type TokenBalance = u64;
    type TokenId = u64;
}

impl pallet_stone_swaps_bridge::Config for TestRuntime {
    type Event = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
    pallet_balances::GenesisConfig::<TestRuntime> {
        balances: vec![
            (ACCOUNT_ID, 10000)
        ],
    }.assimilate_storage(&mut t).unwrap();

	pallet_stone_index::GenesisConfig::<TestRuntime> {
		indexes: vec![(
			INDEX_ID,
			pallet_stone_index::StoneIndex {
				id: INDEX_ID,
				name: "FirstIndex".as_bytes().to_vec(),
				components: vec![
					pallet_stone_index::StoneIndexComponent {
						asset_id: 1,
						weight: 3,
					},
					pallet_stone_index::StoneIndexComponent {
						asset_id: 2,
						weight: 2,
					},
				],
				owner: 100000,
			},
		)]
	}.assimilate_storage(&mut t).unwrap();

    pallet_stone_swaps_bridge::GenesisConfig::<TestRuntime> {
        index_to_token: vec![
            (1, 10001),
            (2, 10002),
        ],
        asset_to_token: vec![
            (1, 101),
            (2, 102),
            (3, 103),
            (4, 104),
            (5, 105),
        ],
    }.assimilate_storage(&mut t).unwrap();
    
    t.into()
}