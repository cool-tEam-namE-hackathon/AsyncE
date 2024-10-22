use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{CHATS, GROUPS},
    user,
};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Chat {
    pub id: u128,
    pub uuid: String,
    pub content: String,
    pub group_id: u128,
    pub username: String,
    pub created_time_unix: u128,
}

#[ic_cdk::query]
pub fn get_chats(group_id: u128) -> Result<Vec<Chat>, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if group.owner != selfname && !group.users.contains(&selfname) {
            return Err(String::from("This user is not in this group!"));
        }

        CHATS.with_borrow(|chats| {
            Ok(chats
                .get(&group_id)
                .cloned()
                .unwrap_or_default()
                .values()
                .cloned()
                .collect::<Vec<_>>())
        })
    })
}
