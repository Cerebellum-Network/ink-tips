//! This is the entry point of the contract. It defines all public messages and events.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)] // for tests in a separate file
#![deny(unused_must_use, unused_variables)]

use ink_lang as ink;

#[ink::contract]
pub mod multi_flipper {
    use ink_prelude::vec::Vec;
    use scale::{Decode, Encode};

    use flippers::{
        entity::FlipperId,
        store::FlipperStore,
    };

    use crate::multi_flipper::flippers::entity::FlipperStatus;

    pub mod flippers;

    // ---- The global state of the contract ----
    #[ink(storage)]
    pub struct MultiFlipper {
        flippers: FlipperStore,
    }

    impl MultiFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                flippers: FlipperStore::default(),
            }
        }
    }
    // ---- End global state ----


    // ---- Flippers ----
    #[ink(event)]
    #[cfg_attr(feature = "std", derive(PartialEq, Debug, scale_info::TypeInfo))]
    pub struct FlipperCreated {
        #[ink(topic)]
        flipper_id: FlipperId,
        #[ink(topic)]
        owner_id: AccountId,
    }

    impl MultiFlipper {
        /// Create a new Flipper owned by the caller.
        #[ink(message)]
        pub fn flipper_create(&mut self) -> Result<FlipperId> {
            self.message_flipper_create()
        }

        /// Flip a Flipper. Only the owner can do this.
        #[ink(message)]
        pub fn flipper_flip(&mut self, flipper_id: FlipperId) -> Result<()> {
            self.message_flipper_flip(flipper_id)
        }

        /// Get the current status of a Flipper.
        #[ink(message)]
        pub fn flipper_get(&self, flipper_id: FlipperId) -> Result<FlipperStatus> {
            self.message_flipper_get(flipper_id)
        }

        /// List all the Flippers.
        ///
        /// The list must be iterated with pagination using the offset and limit parameters and the returned count, until `count <= offset + limit`.
        ///
        /// Optionally, filter the results by owner.
        #[ink(message)]
        pub fn flipper_list(&self, offset: u32, limit: u32, filter_owner_id: Option<AccountId>) -> (Vec<FlipperStatus>, u32) {
            self.message_flipper_list(offset, limit, filter_owner_id)
        }
    }
    // ---- End Flippers ----


    // ---- Utils ----
    /// A token with 10 decimals.
    pub const TOKEN: Balance = 10_000_000_000;

    /// All possible errors.
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        FlipperDoesNotExist,
        Unauthorized,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl From<Error> for ink_env::Error {
        fn from(_: Error) -> Self {
            ink_env::Error::Unknown
        }
    }
    // ---- End Utils ----

    #[cfg(test)]
    mod tests;
}
