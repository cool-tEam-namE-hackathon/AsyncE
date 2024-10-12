use candid::CandidType;
use serde::Deserialize;

use crate::{
    chunk,
    globals::GROUPS,
    primary_key::{self, PrimaryKeyType},
    user,
};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Group {
    pub id: u128,
    pub name: String,
    pub users: Vec<String>,
    pub owner: String,
    pub profile_picture_blob: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GroupQueryResponse {
    pub id: u128,
    pub name: String,
    pub users: Vec<String>,
    pub owner: String,
}

impl Group {
    pub fn new(name: impl Into<String>) -> Self {
        let owner = user::get_selfname().unwrap();
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Group),
            name: name.into(),
            owner: owner.clone(),
            users: Vec::from([owner]),
            profile_picture_blob: Vec::new(),
        }
    }
}

impl From<&Group> for GroupQueryResponse {
    fn from(x: &Group) -> Self {
        Self {
            id: x.id,
            name: x.name.clone(),
            users: x.users.clone(),
            owner: x.owner.clone(),
        }
    }
}

#[ic_cdk::update]
pub fn create_group(name: String) -> u128 {
    user::assert_user_logged_in();

    let group = Group::new(name);
    let group_id = group.id;

    GROUPS.with_borrow_mut(|groups| groups.insert(group.id, group));

    group_id
}

#[ic_cdk::query]
pub fn get_all_groups() -> Vec<GroupQueryResponse> {
    user::assert_user_logged_in();

    let owner = user::get_selfname().unwrap();

    GROUPS.with_borrow(|groups| {
        groups
            .values()
            .filter(|x| x.owner == owner || x.users.contains(&owner))
            .map(GroupQueryResponse::from)
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::query]
pub fn get_group(group_id: u128) -> Option<GroupQueryResponse> {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();
    GROUPS.with_borrow(|groups| {
        let group = groups.get(&group_id);

        if let Some(group) = group.as_ref() {
            if group.owner != selfname && !group.users.contains(&selfname) {
                panic!("This user is not in this group!")
            }
        }

        group.map(GroupQueryResponse::from)
    })
}

#[ic_cdk::update]
pub fn upload_group_profile_picture(group_id: u128, chunk_data: Vec<u8>) {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();
    GROUPS.with_borrow_mut(|groups| {
        let group = groups
            .get_mut(&group_id)
            .expect("Cannot find group with this ID!");

        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }

        group.profile_picture_blob.extend(chunk_data)
    })
}

#[ic_cdk::query]
pub fn get_group_profile_picture_size(group_id: u128) -> u128 {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();
    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .expect("Cannot find group with this ID!");

        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }

        group.profile_picture_blob.len() as u128
    })
}

#[ic_cdk::query]
pub fn get_group_profile_picture_chunk_blob(group_id: u128, index: u128) -> Vec<u8> {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();
    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .expect("Cannot find group with this ID!");

        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }

        group
            .profile_picture_blob
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect()
    })
}
