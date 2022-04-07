use ink_lang as ink;

use crate::multi_flipper::{
    AccountId,
    Error::Unauthorized,
    flippers::entity::{Flipper, FlipperStatus},
    MultiFlipper,
};
use crate::multi_flipper::flippers::entity::FlipperId;

use super::env_utils::{get_accounts, pop_caller, push_caller, push_caller_value};

fn setup() -> (MultiFlipper, FlipperId) {
    let mut contract = MultiFlipper::new();

    push_caller_value(owner_id(), 0);
    let flipper_id = contract.flipper_create().unwrap();
    pop_caller();

    (contract, flipper_id)
}

fn owner_id() -> AccountId { get_accounts().alice }

#[ink::test]
fn flipper_create_works() {
    let (contract, flipper_id) = setup();

    let status = contract.flipper_get(flipper_id)?;
    assert_eq!(status, FlipperStatus {
        flipper_id,
        flipper: Flipper {
            owner_id: owner_id(),
            state: false,
        },
    });
}

#[ink::test]
fn flipper_flip_works() {
    let (mut contract, flipper_id) = setup();

    push_caller(owner_id());
    contract.flipper_flip(flipper_id)?;
    pop_caller();

    let status = contract.flipper_get(flipper_id)?;
    assert_eq!(status.flipper.state, true);
}

#[ink::test]
fn flipper_flip_only_owner() {
    let (mut contract, flipper_id) = setup();
    let not_owner_id = get_accounts().bob;

    push_caller(not_owner_id);
    let err = contract.flipper_flip(flipper_id);
    pop_caller();

    assert_eq!(err, Result::Err(Unauthorized));
}
