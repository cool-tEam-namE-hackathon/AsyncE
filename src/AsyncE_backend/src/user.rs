use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::globals::USERS;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub username: String,
    pub profile_picture_blob: Vec<u8>,
}

pub fn assert_user_logged_in() {
    let principal = ic_cdk::caller();
    if principal != Principal::anonymous() {
        panic!("User needs to login to proceed!")
    }
}

#[ic_cdk::query]
pub fn login() -> Option<User> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).cloned())
}

fn validate_user_register(user: &mut User, principal: Principal) {
    USERS.with_borrow(|users| {
        if !users.contains_key(&principal) {
            panic!("User is already registered!")
        }
    });

    user.username = user.username.trim().to_string();
    if user.username.len() < 3 || user.username.len() > 20 {
        panic!("Username must between 3 to 20 characters!")
    }

    if user.username.chars().any(|x| !x.is_alphanumeric()) {
        panic!("Username contains special characters!")
    }
}

#[ic_cdk::update]
pub fn register(mut user: User) {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    validate_user_register(&mut user, principal);

    USERS.with_borrow_mut(|users| users.insert(principal, user));
}

#[ic_cdk::query]
pub fn get_self() -> Option<User> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).cloned())
}

#[ic_cdk::query]
pub fn get_selfname() -> Option<String> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
}

#[ic_cdk::query]
pub fn get_user(username: String) -> Option<User> {
    assert_user_logged_in();

    USERS.with_borrow(|users| users.values().find(|x| x.username == username).cloned())
}

#[ic_cdk::query]
pub fn get_username() -> Option<String> {
    assert_user_logged_in();

    let principal = ic_cdk::caller();
    USERS.with_borrow(|users| users.get(&principal).map(|x| x.username.clone()))
}
