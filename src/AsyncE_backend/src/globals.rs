use std::{cell::RefCell, collections::BTreeMap};

use candid::Principal;

use crate::{group::Group, user::User};

pub type UserStore = BTreeMap<Principal, User>;
pub type GroupStore = BTreeMap<String, Group>;

thread_local! {
    pub static USERS: RefCell<UserStore> = RefCell::default();
    pub static GROUPS: RefCell<GroupStore> = RefCell::default();
}
