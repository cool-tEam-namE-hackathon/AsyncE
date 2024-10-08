#![allow(non_snake_case)]

pub mod globals;
pub mod group;
pub mod user;

use crate::{group::Group, user::User};
use getrandom::register_custom_getrandom;
use globals::{GROUPS, USERS};

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let users_store = USERS.with_borrow(|users| users.clone());
    let groups_store = GROUPS.with_borrow(|groups| groups.clone());

    ic_cdk::storage::stable_save((users_store, groups_store)).unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (user_store, group_store) = ic_cdk::storage::stable_restore().unwrap();
    USERS.with_borrow_mut(|users| *users = user_store);
    GROUPS.with_borrow_mut(|groups| *groups = group_store);
}

ic_cdk::export_candid!();
