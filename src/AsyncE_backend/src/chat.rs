use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{CHATS, GROUPS},
    user, websocket,
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

        if !group.is_member(&selfname) {
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

#[ic_cdk::update]
pub fn edit_chat(group_id: u128, chat_id: u128, new_content: String) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfuser =
        user::get_selfuser()?.ok_or(String::from("This user does not have a username!"))?;
    if selfuser.subscription.is_none() {
        return Err(String::from("User must be subscribed to use this feature!"));
    }

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if !group.is_member(&selfuser.username) {
            return Err(String::from("This user is not in this group!"));
        }

        CHATS.with_borrow_mut(|chats| {
            let chats = chats
                .get_mut(&group_id)
                .ok_or(String::from("Cannot find any chat from this group!"))?;

            let chat = chats
                .get_mut(&chat_id)
                .ok_or(String::from("Cannot get chat with this ID!"))?;
            chat.content = new_content;

            websocket::broadcast_edit_chat(group_id, chat_id, chat.content.clone());

            Ok(())
        })
    })
}

#[ic_cdk::update]
pub fn delete_chat(group_id: u128, chat_id: u128) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfuser =
        user::get_selfuser()?.ok_or(String::from("This user does not have a username!"))?;
    if selfuser.subscription.is_none() {
        return Err(String::from("User must be subscribed to use this feature!"));
    }

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if !group.is_member(&selfuser.username) {
            return Err(String::from("This user is not in this group!"));
        }

        CHATS.with_borrow_mut(|chats| {
            let chats = chats
                .get_mut(&group_id)
                .ok_or(String::from("Cannot find any chat from this group!"))?;

            chats
                .remove(&chat_id)
                .ok_or(String::from("Cannot get chat with this ID!"))?;
            websocket::broadcast_delete_chat(group_id, chat_id);

            Ok(())
        })
    })
}
