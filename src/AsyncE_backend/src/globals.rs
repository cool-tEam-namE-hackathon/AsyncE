use std::{cell::RefCell, collections::BTreeMap};

use candid::Principal;

use crate::{group::Group, user::User};

thread_local! {
    pub static USERS: RefCell<BTreeMap<Principal, User>> = RefCell::default();
    pub static GROUPS: RefCell<BTreeMap<String, Group>> = RefCell::default();
}
