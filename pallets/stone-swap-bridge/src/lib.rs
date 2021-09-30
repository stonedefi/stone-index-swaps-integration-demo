#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{SaturatedConversion};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_stone_index::Config + pallet_swaps::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn index_to_token)]
    pub(super) type IndexToToken<T> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet_stone_index::Config>::IndexId,
        <T as pallet_fungible::Config>::TokenId,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn token_to_index)]
    pub(super) type TokenToIndex<T> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet_fungible::Config>::TokenId,
        <T as pallet_stone_index::Config>::IndexId,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn asset_to_token)]
    pub(super) type AssetToToken<T> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet_assets::Config>::AssetId,
        <T as pallet_fungible::Config>::TokenId,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn token_to_asset)]
    pub(super) type TokenToAsset<T> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet_fungible::Config>::TokenId,
        <T as pallet_assets::Config>::AssetId,
        ValueQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub index_to_token: Vec<(T::IndexId, T::TokenId)>,
        pub asset_to_token: Vec<(T::AssetId, T::TokenId)>,
    } 

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
			for (k, v) in &self.index_to_token {
				<IndexToToken<T>>::insert(k, v);
				<TokenToIndex<T>>::insert(v, k);
			}
			for (k, v) in &self.asset_to_token {
				<AssetToToken<T>>::insert(k, v);
				<TokenToAsset<T>>::insert(v, k);
			}
        }
    }

    #[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				index_to_token: Default::default(),
				asset_to_token: Default::default(),
			}
		}
	}

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn deposit_index_to_swap(origin: OriginFor<T>, index_id: T::IndexId, amount: T::Balance) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let token_id = Self::index_to_token(index_id);
            let token_amount = Self::convert(amount);

            // mint token_id
            pallet_fungible::Module::<T>::mint(token_id, who.clone(), token_amount).unwrap();
            // burn index_id
            pallet_stone_index::Module::<T>::_transfer(index_id, who.clone(), T::AccountId::default(), amount);

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_index_from_swap(origin: OriginFor<T>, token_id: T::TokenId, amount: T::TokenBalance) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let index_id = Self::token_to_index(token_id);
            let index_amount = Self::unconvert(amount);

            // burn token_id
            pallet_fungible::Module::<T>::burn(token_id, who.clone(), amount).unwrap();
            // mint index_id
            pallet_stone_index::Module::<T>::_mint(index_id, who.clone(), index_amount);

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn deposit_asset_to_swap(origin: OriginFor<T>, asset_id: T::AssetId, amount: T::Balance) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let token_id = Self::asset_to_token(asset_id);
            let token_amount = Self::convert(amount);

            // burn asset_id
            pallet_assets::Module::<T>::burn(asset_id, who.clone(), amount);
            // mint token_id
            pallet_fungible::Module::<T>::mint(token_id, who.clone(), token_amount).unwrap();

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_token_from_swap(origin: OriginFor<T>, token_id: T::TokenId, amount: T::TokenBalance) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let asset_id = Self::token_to_asset(token_id);
            let asset_amount = Self::unconvert(amount);

            // burn token_id
            pallet_fungible::Module::<T>::burn(token_id, who.clone(), amount).unwrap();
            // mint asset_id
            pallet_assets::Module::<T>::mint(asset_id, who.clone(), asset_amount);

            Ok(().into())
        }

    }

    impl<T: Config> Pallet<T> {
        fn convert(amount: T::Balance) -> T::TokenBalance {
            let m = amount.saturated_into::<u64>();
            m.saturated_into()
        }

        fn unconvert(amount: T::TokenBalance) -> T::Balance {
            let m = amount.saturated_into::<u64>();
            m.saturated_into()
        }
    }
}
