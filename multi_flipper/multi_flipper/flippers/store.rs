//! The store where to create and access Flippers by ID.

use ink_storage::{
    collections::Vec as InkVec,
    traits,
};

use crate::multi_flipper::{AccountId, Error::FlipperDoesNotExist, Result};

use super::entity::{Flipper, FlipperId};

#[derive(traits::SpreadLayout, Default)]
#[cfg_attr(feature = "std", derive(traits::StorageLayout, Debug))]
pub struct FlipperStore(pub InkVec<Flipper>);

impl FlipperStore {
    pub fn create(
        &mut self,
        owner_id: AccountId,
    ) -> FlipperId {
        let flipper = Flipper::new(owner_id);
        let flipper_id = self.0.len();
        self.0.push(flipper);
        flipper_id
    }

    pub fn get(&self, flipper_id: FlipperId) -> Result<&Flipper> {
        self.0.get(flipper_id).ok_or(FlipperDoesNotExist)
    }

    pub fn get_mut(&mut self, flipper_id: FlipperId) -> Result<&mut Flipper> {
        self.0.get_mut(flipper_id).ok_or(FlipperDoesNotExist)
    }
}
