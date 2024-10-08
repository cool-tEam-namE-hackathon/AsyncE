use std::{cell::RefCell, collections::HashMap};

use candid::Principal;

use crate::{group::Group, user::User};

thread_local! {
    pub static USERS: RefCell<HashMap<Principal, User>> = RefCell::default();
    pub static GROUPS: RefCell<HashMap<String, Group>> = RefCell::default();
}
