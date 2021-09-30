use crate::mock::*;
use frame_support::{assert_ok};

#[test]
fn test_deposit_and_withdraw_index_with_swap() {
    new_test_ext().execute_with(|| {
        let token_id: u64 = StoneSwapBridge::index_to_token(INDEX_ID);
        StoneIndex::_mint(INDEX_ID, ACCOUNT_ID, 50000);

        assert_ok!(StoneSwapBridge::deposit_index_to_swap(Origin::signed(ACCOUNT_ID), INDEX_ID, 20000));
        assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 30000);
        assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 20000);

        assert_ok!(StoneSwapBridge::withdraw_index_from_swap(Origin::signed(ACCOUNT_ID), token_id, 10000));
        assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 40000);
        assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 10000);
    });
}

#[test]
fn test_adds_index_liquidity_to_swap() {
    new_test_ext().execute_with(|| {
        let token_id: u64 = StoneSwapBridge::index_to_token(INDEX_ID);
        StoneIndex::_mint(INDEX_ID, ACCOUNT_ID, 50000);

        assert_ok!(StoneSwapBridge::deposit_index_to_swap(Origin::signed(ACCOUNT_ID), INDEX_ID, 20000));
        assert_eq!(StoneIndex::index_balances((INDEX_ID, ACCOUNT_ID)), 30000);
        assert_eq!(Fungible::balance_of((token_id, ACCOUNT_ID)), 20000);

        assert_ok!(Swaps::create_swap(Origin::signed(ACCOUNT_ID), token_id));
        assert_ok!(Swaps::add_liquidity(Origin::signed(ACCOUNT_ID), 0, 200, 0, 20, 100));
        assert_ok!(Swaps::add_liquidity(Origin::signed(ACCOUNT_ID), 0, 100, 100, 10, 100));
    });
}