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

## Setup

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

## Tests

After cloning the repository, run `cargo test` to build the packages and run
the tests.

[substrate]: https://github.com/paritytech/substrate

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

