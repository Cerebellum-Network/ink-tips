//! The data structure of Flippers.

use ink_storage::traits::{PackedLayout, SpreadLayout};
use scale::{Decode, Encode};

use crate::multi_flipper::{AccountId, Error::Unauthorized, Result};

pub type FlipperId = u32;

/// Flipper is the data structure that is persisted in storage.
#[derive(Clone, PartialEq, Encode, Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct Flipper {
    pub owner_id: AccountId,
    pub state: bool,
}

/// FlipperStatus is the data structure returned by public getters.
///
/// This may contains more details that the Flipper structure, depending on the current time or computed values or other data.
#[derive(Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct FlipperStatus {
    pub flipper_id: FlipperId,
    pub flipper: Flipper,
}

impl Flipper {
    pub fn new(owner_id: AccountId) -> Self {
        Self { owner_id, state: false }
    }

    pub fn flip(&mut self) -> Result<()> {
        self.state = !self.state;
        Ok(())
    }

    pub fn only_owner(&self, caller: AccountId) -> Result<()> {
        if self.owner_id == caller { Ok(()) } else { Err(Unauthorized) }
    }
}