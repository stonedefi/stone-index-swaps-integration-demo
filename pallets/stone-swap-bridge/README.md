This is a demo showing how to integrate stone index with other dex (`pallet-swap` in this sample).

Firstly, add the following pallets into the dependency section.
```
pallet-stone-index = { git = 'https://github.com/stonedefi/stoneindex-node', default-features = false, version = '3.0.0' }
pallet-assets = { git = 'https://github.com/stonedefi/stoneindex-node', default-features = false, version = '2.1.0' }
pallet-swaps = { git = 'https://github.com/stonedefi/pallet-swaps', default-features = false, version = '0.1.1' }

...
```

Then implement the config for the pallets
```
impl pallet_assets::Config for TestRuntime {
    type Event = ();
    type Balance = u64;
    type AssetId = u64;
}

impl pallet_stone_index::Config for TestRuntime {
    type Event = ();
}

...
```