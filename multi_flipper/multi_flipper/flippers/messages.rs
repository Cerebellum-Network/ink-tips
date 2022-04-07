//! The implementation of public messages handlers.
//!
//! This code can
//   access the contract environment with `Self::env()`, and it can access the storage of other modules
//   using `self.some_store`. Importantly, multiple stores can be accessed simultaneously while respecting Rust borrow safety.

use ink_lang::{EmitEvent, StaticEnv};
use ink_prelude::vec::Vec;

use crate::multi_flipper::{AccountId, FlipperCreated, MultiFlipper, Result};

use super::entity::{Flipper, FlipperId, FlipperStatus};

// This block is mixed with the public functions in lib.rs and other messages.rs files,
// so the implementations can access `self` which is the main contract storage.
//
// These implementations match the public function definitions but without any ink macro and with the `message_` prefix to distinguish them.
impl MultiFlipper {
    pub fn message_flipper_create(&mut self) -> Result<FlipperId> {
        let owner_id = Self::env().caller();
        let flipper_id = self.flippers.create(owner_id);
        Self::env().emit_event(FlipperCreated { flipper_id, owner_id });
        Ok(flipper_id)
    }

    pub fn message_flipper_flip(&mut self, flipper_id: FlipperId) -> Result<()> {
        let flipper = self.flippers.get_mut(flipper_id)?;
        Self::only_flipper_owner(flipper)?;
        flipper.flip()?;
        Ok(())
    }

    pub fn message_flipper_get(&self, flipper_id: FlipperId) -> Result<FlipperStatus> {
        let flipper = self.flippers.get(flipper_id)?.clone();
        Ok(FlipperStatus { flipper_id, flipper })
    }

    pub fn message_flipper_list(&self, offset: u32, limit: u32, filter_owner_id: Option<AccountId>) -> (Vec<FlipperStatus>, u32) {
        let mut statuses = Vec::with_capacity(limit as usize);
        for flipper_id in offset..offset + limit {
            let flipper = match self.flippers.0.get(flipper_id) {
                None => break, // No more items, stop.
                Some(flipper) => flipper,
            };
            // Apply the filter if given.
            if let Some(owner_id) = filter_owner_id {
                if owner_id != flipper.owner_id {
                    continue; // Skip non-matches.
                }
            }
            // Include the current status of matched items.
            let status = FlipperStatus {
                flipper_id,
                flipper: flipper.clone(),
            };
            statuses.push(status);
        }
        let count = self.flippers.0.len();
        (statuses, count)
    }

    fn only_flipper_owner(flipper: &Flipper) -> Result<()> {
        let caller = Self::env().caller();
        flipper.only_owner(caller)
    }
}
