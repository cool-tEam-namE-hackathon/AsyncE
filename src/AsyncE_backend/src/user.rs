use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::globals::USERS;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub username: Option<String>,
    pub profile_picture_blob: Vec<u8>,
}

pub fn assert_user_logged_in() {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    if USERS
        .with_borrow(|users| users.get(&principal).and_then(|x| x.username.clone()))
        .is_none()
    {
        panic!("User needs to have a username to proceed!")
    }
}

#[ic_cdk::query]
pub fn login() -> Option<User> {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).cloned())
}

fn validate_user_register(user: &mut User, principal: Principal) {
    USERS.with_borrow(|users| {
        if !users.contains_key(&principal) {
            panic!("User is already registered!")
        }
    });

    if user.username.is_none() {
        panic!("Username must be filled!")
    }

    let username = user.username.as_ref().unwrap().trim().to_string();
    if username.len() < 3 || username.len() > 20 {
        panic!("Username must between 3 to 20 characters!")
    }

    if username.chars().any(|x| !x.is_alphanumeric()) {
        panic!("Username contains special characters!")
    }

    USERS.with_borrow(|users| {
        if users.values().any(|x| {
            x.username
                .as_ref()
                .map(|x| x.eq_ignore_ascii_case(&username))
                .unwrap_or(false)
        }) {
            panic!("User is already registered!")
        }
    });

    user.username = Some(username);
}

#[ic_cdk::update]
pub fn register(mut user: User) {
    // we don't use `assert_user_logged_in` since that function
    // also checks for `null username` which we definitely have
    // at this moment since it's "registering"
    let principal = ic_cdk::caller();
    if principal != Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    validate_user_register(&mut user, principal);

    USERS.with_borrow_mut(|users| users.insert(principal, user));
}

#[ic_cdk::query]
pub fn get_self() -> Option<User> {
    // we don't use `assert_user_logged_in` since that function
    // also checks for `null username`
    // and at this point when user refreshes the page when on `creating username` part
    // this function won't return err
    let principal = ic_cdk::caller();
    if principal != Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).cloned())
}

#[ic_cdk::query]
pub fn get_selfname() -> Option<String> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).and_then(|x| x.username.clone()))
}

#[ic_cdk::query]
pub fn get_user(username: String) -> Option<User> {
    assert_user_logged_in();

    let username = Some(username);
    USERS.with_borrow(|users| users.values().find(|x| x.username == username).cloned())
}

#[ic_cdk::query]
pub fn get_username() -> Option<String> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).and_then(|x| x.username.clone()))
}

#[ic_cdk::query]
pub fn query_username(keyword: String) -> Vec<String> {
    assert_user_logged_in();

    USERS.with_borrow(|users| {
        users
            .values()
            .filter(|x| x.username.is_some())
            .filter(|x| x.username.as_ref().unwrap().eq_ignore_ascii_case(&keyword))
            .map(|x| x.username.as_ref().unwrap().clone())
            .collect::<Vec<_>>()
    })
}
