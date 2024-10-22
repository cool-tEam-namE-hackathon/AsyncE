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
    pub created_time_unix: u128,
    pub profile_picture_blob: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GroupQueryResponse {
    pub id: u128,
    pub name: String,
    pub users: Vec<String>,
    pub owner: String,
    pub created_time_unix: u128,
}

impl Group {
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let owner = user::get_selfname_force()?;

        Ok(Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Group),
            name: name.into(),
            owner: owner.clone(),
            users: Vec::from([owner]),
            created_time_unix: ic_cdk::api::time() as u128,
            profile_picture_blob: Vec::new(),
        })
    }
}

impl From<&Group> for GroupQueryResponse {
    fn from(x: &Group) -> Self {
        Self {
            id: x.id,
            name: x.name.clone(),
            users: x.users.clone(),
            owner: x.owner.clone(),
            created_time_unix: x.created_time_unix,
        }
    }
}

#[ic_cdk::update]
pub fn create_group(name: String) -> Result<u128, String> {
    user::assert_user_logged_in()?;

    let group = Group::new(name)?;
    let group_id = group.id;

    GROUPS.with_borrow_mut(|groups| groups.insert(group.id, group));

    Ok(group_id)
}

#[ic_cdk::update]
pub fn get_all_groups() -> Result<Vec<GroupQueryResponse>, String> {
    user::assert_user_logged_in()?;

    let owner = user::get_selfname_force()?;

    Ok(GROUPS.with_borrow(|groups| {
        groups
            .values()
            .filter(|x| x.owner.eq_ignore_ascii_case(&owner) || x.users.contains(&owner))
            .map(GroupQueryResponse::from)
            .collect::<Vec<_>>()
    }))
}

#[ic_cdk::query]
pub fn get_group(group_id: u128) -> Result<Option<GroupQueryResponse>, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow(|groups| {
        let group = groups.get(&group_id);

        if let Some(group) = group.as_ref() {
            if group.owner != selfname && !group.users.contains(&selfname) {
                return Err(String::from("This user is not in this group!"));
            }
        }

        Ok(group.map(GroupQueryResponse::from))
    })
}

#[ic_cdk::update]
pub fn upload_group_profile_picture(
    group_id: u128,
    chunk_data: Vec<u8>,
    chunk_index: u128,
    total_data_length: u128,
) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow_mut(|groups| {
        let group = groups
            .get_mut(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if group.owner != selfname && !group.users.contains(&selfname) {
            return Err(String::from("This user is not in this group!"));
        }

        if group.profile_picture_blob.capacity() != total_data_length as usize {
            group
                .profile_picture_blob
                .reserve_exact(total_data_length as usize);
        }

        let offset = chunk_index as usize * chunk::MB;
        group
            .profile_picture_blob
            .splice(offset..offset, chunk_data);

        Ok(())
    })
}

#[ic_cdk::query]
pub fn get_group_profile_picture_size(group_id: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if group.owner != selfname && !group.users.contains(&selfname) {
            return Err(String::from("This user is not in this group!"));
        }

        Ok(group.profile_picture_blob.len() as u128)
    })
}

#[ic_cdk::query]
pub fn get_group_profile_picture_chunk_blob(
    group_id: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if group.owner != selfname && !group.users.contains(&selfname) {
            return Err(String::from("This user is not in this group!"));
        }

        Ok(group
            .profile_picture_blob
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect())
    })
}
