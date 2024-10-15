use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{chunk, globals::USERS};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub username: String,
    pub profile_picture_blob: Vec<u8>,
}

pub fn assert_user_logged_in() -> Result<(), String> {
    assert_user_logged_in_from(ic_cdk::caller())
}

pub fn assert_user_logged_in_from(principal: Principal) -> Result<(), String> {
    if principal == Principal::anonymous() {
        return Err(String::from("User needs to sign in to proceed!"));
    }

    if USERS
        .with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
        .is_none()
    {
        return Err(String::from("User needs to have a username to proceed!"));
    }

    Ok(())
}

#[ic_cdk::query]
pub fn get_user_credentials() -> Result<Option<String>, String> {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        return Err(String::from("User needs to sign in to proceed!"));
    }

    let principal = ic_cdk::caller();
    Ok(USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone())))
}

fn validate_user_register(name: &str, principal: Principal) -> Result<(), String> {
    USERS.with_borrow(|users| {
        if users.contains_key(&principal) {
            return Err(String::from("User is already registered!"));
        }

        Ok(())
    })?;

    let username = name.trim().to_string();
    if username.len() < 3 || username.len() > 20 {
        return Err(String::from("Username must between 3 to 20 characters!"));
    }

    if username.chars().any(|x| !x.is_alphanumeric()) {
        return Err(String::from("Username contains special characters!"));
    }

    USERS.with_borrow(|users| {
        if users
            .values()
            .any(|x| x.username.eq_ignore_ascii_case(&username))
        {
            return Err(String::from("User is already registered!"));
        }

        Ok(())
    })?;

    Ok(())
}

#[ic_cdk::update]
pub fn register(name: String) -> Result<(), String> {
    // we don't use `assert_user_logged_in` since that function
    // also checks for `null username` which we definitely have
    // at this moment since it's "registering"
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        return Err(String::from("User needs to sign in to proceed!"));
    }

    validate_user_register(&name, principal)?;

    let user = User {
        username: name,
        profile_picture_blob: Vec::new(),
    };
    USERS.with_borrow_mut(|users| users.insert(principal, user));

    Ok(())
}

#[ic_cdk::query]
pub fn get_selfname() -> Result<Option<String>, String> {
    assert_user_logged_in()?;

    let principal = ic_cdk::caller();

    Ok(USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone())))
}

pub fn get_selfname_force() -> Result<String, String> {
    get_selfname()?.ok_or(String::from("This user does not have a username!"))
}

#[ic_cdk::query]
pub fn validate_username(name: String) -> Result<bool, String> {
    assert_user_logged_in()?;

    Ok(USERS.with_borrow(|users| {
        users
            .values()
            .any(|x| x.username.eq_ignore_ascii_case(&name))
    }))
}

#[ic_cdk::update]
pub fn upload_profile_picture(chunk_data: Vec<u8>) -> Result<(), String> {
    assert_user_logged_in()?;

    let principal = ic_cdk::caller();
    USERS.with_borrow_mut(|users| {
        users
            .get_mut(&principal)
            .ok_or(String::from("Cannot find user with this principal!"))?
            .profile_picture_blob
            .extend(chunk_data);

        Ok(())
    })
}

#[ic_cdk::query]
pub fn get_profile_picture_size() -> Result<u128, String> {
    assert_user_logged_in()?;

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| {
        users
            .get(&principal)
            .map(|x| x.profile_picture_blob.len() as u128)
            .ok_or(String::from("Cannot find user with this principal!"))
    })
}

#[ic_cdk::query]
pub fn get_profile_picture_chunk_blob(index: u128) -> Result<Vec<u8>, String> {
    assert_user_logged_in()?;

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| {
        users
            .get(&principal)
            .map(|x| {
                x.profile_picture_blob
                    .iter()
                    .skip(index as usize * chunk::MB)
                    .take(chunk::MB)
                    .cloned()
                    .collect()
            })
            .ok_or(String::from("Cannot find user with this principal!"))
    })
}
