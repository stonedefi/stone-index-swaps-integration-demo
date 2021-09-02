#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests {
    use frame_support::{construct_runtime, parameter_types, assert_ok};
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
    type Block = frame_system::mocking::MockBlock<TestRuntime>;
    
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

    fn swap_account_for_asset(index_id: u64) -> u64 {
        match index_id {
            10001 => 78694532,
            10002 => 67534190,
            _ => 0,
        }
    }

    fn token_id_for_index_id(index_id: u64) -> u64 {
        match index_id {
            10001 => 11,
            10002 => 12,
            _ => 0,
        }
    }

    fn deposit_index_to_swap(account: u64, index_id: u64, amount: u64) {
        let to = swap_account_for_asset(index_id);
        let token_id = token_id_for_index_id(index_id);
        StoneIndex::_transfer(index_id, account, to, amount);
        Fungible::mint(token_id, account, amount).unwrap();
    }

    fn withdraw_index_from_swap(account: u64, index_id: u64, amount: u64) {
        let from = swap_account_for_asset(index_id);
        let token_id = token_id_for_index_id(index_id);
        StoneIndex::_transfer(index_id, from, account, amount);
        Fungible::burn(token_id, account, amount).unwrap();
    }

    const ACCOUNT_ID: u64 = 6798534;
    const INDEX_ID: u64 = 10001;

	fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap().into();
        pallet_balances::GenesisConfig::<TestRuntime> {
            balances: vec![
                (ACCOUNT_ID, 10000)
            ],
        }.assimilate_storage(&mut t).unwrap();
        t.into()
	}

	#[test]
	fn test_deposit_and_withdraw_index_with_swap() {
		new_test_ext().execute_with(|| {
            let token_id: u64 = token_id_for_index_id(INDEX_ID);
            StoneIndex::_mint(INDEX_ID, ACCOUNT_ID, 50000);

            deposit_index_to_swap(ACCOUNT_ID, INDEX_ID, 20000);
            assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 30000);
            assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 20000);

            withdraw_index_from_swap(ACCOUNT_ID, INDEX_ID, 10000);
            assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 40000);
            assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 10000);
		});
	}

    #[test]
    fn test_adds_index_liquidity_to_swap() {
        new_test_ext().execute_with(|| {
            let token_id: u64 = token_id_for_index_id(INDEX_ID);
            StoneIndex::_mint(INDEX_ID, ACCOUNT_ID, 50000);

            deposit_index_to_swap(ACCOUNT_ID, INDEX_ID, 20000);
            assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 30000);
            assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 20000);


            assert_ok!(Swaps::create_swap(Origin::signed(ACCOUNT_ID), token_id));
            assert_ok!(Swaps::add_liquidity(Origin::signed(ACCOUNT_ID), 0, 200, 0, 20, 100));
            assert_ok!(Swaps::add_liquidity(Origin::signed(ACCOUNT_ID), 0, 100, 100, 10, 100));
        });
    }
}