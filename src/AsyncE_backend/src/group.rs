use candid::CandidType;
use serde::Deserialize;
use uuid::Uuid;

use crate::{globals::GROUPS, user};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub users: Vec<String>,
    pub owner: String,
    pub profile_picture_blob: Vec<u8>,
}

impl Group {
    pub fn new(name: impl Into<String>, profile_picture_blob: Vec<u8>) -> Self {
        let owner = user::get_selfname().unwrap();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            owner: owner.clone(),
            users: Vec::from([owner]),
            profile_picture_blob,
        }
    }
}

#[ic_cdk::update]
pub fn create_group(name: String, profile_picture_blob: Vec<u8>) {
    user::assert_user_logged_in();

    let group = Group::new(name, profile_picture_blob);

    GROUPS.with_borrow_mut(|groups| groups.insert(group.id.clone(), group));
}

#[ic_cdk::query]
pub fn get_all_groups() -> Vec<Group> {
    user::assert_user_logged_in();

    let owner = user::get_selfname().unwrap();

    GROUPS.with_borrow(|groups| {
        groups
            .values()
            .filter(|x| x.owner == owner || x.users.contains(&owner))
            .cloned()
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::query]
pub fn get_group(id: String) -> Option<Group> {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();
    let group = GROUPS.with_borrow(|groups| groups.get(&id).cloned());
    if let Some(group) = group.as_ref() {
        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }
    }

    group
}
