This is a bridge pallet showing how to integrate stone index with other dex (`pallet-swaps` in this sample). The keypoint is to provide some APIs for transferring across the ledgers. To acheive this, two maps are used for mapping the IDs in `pallet-stone-index` and `pallet-swaps`.

The following steps show how to use this bridge for a major scenario: a user has some USDT & STN tokens in the DEX and wants to exchange to an USDT+STN index.

1. call `StoneSwapBridge::withdraw_token_from_swap` to transfer the tokens to Stone Index.
1. call `StoneIndex::buy_index` to buy the index with the tokens.
1. call `StoneSwapBridge::deposit_index_to_swap` to transfer back the index as a token to DEX for further trade.

This scenario is covered in the test case `test_buy_index_with_token`.
