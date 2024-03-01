#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn something)]

    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored(u32, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidValue,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // To test the case follow these instructions:
    //
    // 1) Execute with (5, true). No issues and the value will be set to Some(5).
    // 2) Execute with (11, false). Exception is raised and the value will be set to None. This modifies the storage even with a failed transaction.
    // 3) Execute with (2, true). No issues and the value will be set to Some(2).
    // 4) Execute with (7, false). No errors, but the value will be set to None. This shows that the value was firstly modified by the lambda and later read by the extrinsic.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::feeless_if(| origin: &OriginFor<T>, _something: &u32, _force: &bool | -> bool {
            // Here we modify the storage. If there was something on storage it is now cleared
            <Something<T>>::take().is_some()
        })]
        #[pallet::weight({600_000})]
        pub fn do_something(origin: OriginFor<T>, something: u32, force: bool) -> DispatchResult {
            let who = ensure_signed(origin)?;
            if something >= 10 {
                return Err(Error::<T>::InvalidValue.into())
            }

            match <Something<T>>::get() {
                None if !force => {}
                _ => {
                    <Something<T>>::put(something);
                    Self::deposit_event(Event::SomethingStored(something, who));
                }
            }

            Ok(())
        }
    }
}
