use std::collections::BTreeMap;

use candid::CandidType;
use serde::Deserialize;

use crate::globals::PRIMARY_KEY_CONTAINERS;

#[derive(PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Clone)]
pub enum PrimaryKeyType {
    Group,
    Video,
}

pub type PrimaryKeyContainer = BTreeMap<PrimaryKeyType, u128>;

pub fn get_primary_key(ty: PrimaryKeyType) -> u128 {
    PRIMARY_KEY_CONTAINERS.with_borrow_mut(|primary_key_containers| {
        let entry = primary_key_containers.entry(ty).or_insert(1);
        let copy = *entry;
        *entry += 1;
        copy
    })
}
