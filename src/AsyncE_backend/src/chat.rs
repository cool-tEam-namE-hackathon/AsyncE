use std::time::{SystemTime, UNIX_EPOCH};

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{CHATS, GROUPS},
    primary_key::{self, PrimaryKeyType},
    user,
    websocket::{self, WebsocketEventMessage},
};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Chat {
    pub id: u128,
    pub content: String,
    pub username: String,
    pub created_time_unix: u128,
}

impl Chat {
    pub fn new(username: String, content: String) -> Self {
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Chat),
            content,
            username,
            created_time_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis(),
        }
    }
}

#[ic_cdk::update]
pub fn add_chat(group_id: u128, content: String) {
    let selfname = user::get_selfname().unwrap();
    let group = GROUPS
        .with_borrow(|groups| groups.get(&group_id).cloned())
        .expect("Cannot find group with this ID!");

    if group.owner != selfname && !group.users.contains(&selfname) {
        panic!("This user is not in this group!")
    }

    let chat = Chat::new(selfname, content);
    CHATS.with_borrow_mut(|chats| {
        chats
            .entry(group_id)
            .or_default()
            .insert(chat.id, chat.clone());

        websocket::broadcast_websocket_message(WebsocketEventMessage::new_chat(chat));
    })
}

#[ic_cdk::query]
pub fn get_chats(group_id: u128) -> Vec<Chat> {
    let selfname = user::get_selfname().unwrap();
    let group = GROUPS
        .with_borrow(|groups| groups.get(&group_id).cloned())
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
}
