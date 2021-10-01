# Integrate Stone Index with DEX

As a complete scenario, users should be able to trade their tokens such as USDT/USDC/DOT and indexes freely in a chain. Specifically, it contains the following user stories:
1. Users can buy the index shares with the tokens in the chain.
1. Users can sell the index shares back to the tokens in the chain.
1. Users can trade the index shares as normal tokens in the DEX.


But due to the nature of the pallets, `stone-index` and DEXes rely on their own ledgers which have no knowledge of others. This prevents users from buying/selling indexes in `stone-index` with tokens in DEX.

To solve the above issue, we wrote this bridge pallet showing how to integrate stone index with other dex (`pallet-swaps` in this sample). The keypoint is to provide some APIs for allowing transfer across the ledgers. And the different IDs in `pallet-stone-index` and `pallet-swaps` are correlated by the map storage. 

The following steps show how to use this bridge for a major scenario: a user has some USDT & STN tokens in the DEX and wants to exchange to an USDT+STN index.

1. call `StoneSwapBridge::withdraw_token_from_swap` to transfer the tokens to Stone Index.
1. call `StoneIndex::buy_index` to buy the index with the tokens.
1. call `StoneSwapBridge::deposit_index_to_swap` to transfer back the index as a token to DEX for further trade.

This scenario is covered in the test case `test_buy_index_with_token`.
