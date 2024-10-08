#![allow(non_snake_case)]

use candid::{CandidType, Principal};
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub username: String,
}

thread_local! {
    pub static USERS: RefCell<HashMap<Principal, User>> = RefCell::default();
}

pub fn assert_user_logged_in() {
    let principal = ic_cdk::api::caller();
    assert!(
        principal != Principal::anonymous(),
        "User needs to login to proceed."
    );
}

#[ic_cdk::query]
pub fn login() -> Option<User> {
    assert_user_logged_in();

    let principal = ic_cdk::api::caller();
    USERS.with(|users| users.borrow().get(&principal).cloned())
}

#[ic_cdk::update]
pub fn register(user: User) {
    assert_user_logged_in();

    let principal = ic_cdk::api::caller();
    USERS.with(|users| {
        if users.borrow().contains_key(&principal) {
            panic!("user is already registered!")
        }

        users.borrow_mut().insert(principal, user);
    });
}
