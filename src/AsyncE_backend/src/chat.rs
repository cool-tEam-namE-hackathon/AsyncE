use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{CHATS, GROUPS},
    primary_key::{self, PrimaryKeyType},
    user,
};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Chat {
    pub id: u128,
    pub content: String,
    pub group_id: u128,
    pub username: String,
    pub created_time_unix: u128,
}

impl Chat {
    pub fn new(group_id: u128, username: String, content: String) -> Self {
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Chat),
            content,
            group_id,
            username,
            created_time_unix: ic_cdk::api::time() as u128,
        }
    }
}

#[ic_cdk::query]
pub fn get_chats(group_id: u128) -> Vec<Chat> {
    let selfname = user::get_selfname().unwrap();
    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .expect("Cannot find group with this ID!");
        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }

        CHATS.with_borrow(|chats| {
            chats
                .get(&group_id)
                .cloned()
                .unwrap_or_default()
                .values()
                .cloned()
                .collect::<Vec<_>>()
        })
    })
}
