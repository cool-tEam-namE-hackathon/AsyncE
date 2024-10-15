use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{chunk, globals::USERS};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub username: String,
    pub profile_picture_blob: Vec<u8>,
}

pub fn assert_user_logged_in() {
    assert_user_logged_in_from(ic_cdk::caller());
}

pub fn assert_user_logged_in_from(principal: Principal) {
    if principal == Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    if USERS
        .with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
        .is_none()
    {
        panic!("User needs to have a username to proceed!")
    }
}

#[ic_cdk::query]
pub fn get_user_credentials() -> Option<String> {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
}

fn validate_user_register(name: &str, principal: Principal) {
    USERS.with_borrow(|users| {
        if users.contains_key(&principal) {
            panic!("User is already registered!")
        }
    });

    let username = name.trim().to_string();
    if username.len() < 3 || username.len() > 20 {
        panic!("Username must between 3 to 20 characters!")
    }

    if username.chars().any(|x| !x.is_alphanumeric()) {
        panic!("Username contains special characters!")
    }

    USERS.with_borrow(|users| {
        if users
            .values()
            .any(|x| x.username.eq_ignore_ascii_case(&username))
        {
            panic!("User is already registered!")
        }
    });
}

#[ic_cdk::update]
pub fn register(name: String) {
    // we don't use `assert_user_logged_in` since that function
    // also checks for `null username` which we definitely have
    // at this moment since it's "registering"
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    validate_user_register(&name, principal);

    let user = User {
        username: name,
        profile_picture_blob: Vec::new(),
    };
    USERS.with_borrow_mut(|users| users.insert(principal, user));
}

#[ic_cdk::query]
pub fn get_selfname() -> Option<String> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
}

#[ic_cdk::query]
pub fn query_username(keyword: String) -> Vec<String> {
    assert_user_logged_in();

    USERS.with_borrow(|users| {
        users
            .values()
            .filter(|x| x.username.eq_ignore_ascii_case(&keyword))
            .map(|x| x.username.clone())
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::query]
pub fn validate_username(name: String) -> bool {
    assert_user_logged_in();
    ic_cdk::println!("finding username: {}", name);
    ic_cdk::println!(
        "all usernames: {:?}",
        USERS.with_borrow(|users| users
            .values()
            .map(|x| x.username.clone())
            .collect::<Vec<_>>())
    );
    USERS.with_borrow(|users| {
        users
            .values()
            .find(|x| x.username.eq_ignore_ascii_case(&name))
            .is_some()
    })
}

#[ic_cdk::query]
pub fn get_all_usernames() -> Vec<String> {
    USERS.with_borrow(|users| {
        users
            .values()
            .map(|x| x.username.clone())
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::update]
pub fn upload_profile_picture(chunk_data: Vec<u8>) {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow_mut(|users| {
        users
            .get_mut(&principal)
            .unwrap()
            .profile_picture_blob
            .extend(chunk_data);
    });
}

#[ic_cdk::query]
pub fn get_profile_picture_size() -> u128 {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).unwrap().profile_picture_blob.len() as u128)
}

#[ic_cdk::query]
pub fn get_profile_picture_chunk_blob(index: u128) -> Vec<u8> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| {
        users
            .get(&principal)
            .unwrap()
            .profile_picture_blob
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect()
    })
}
