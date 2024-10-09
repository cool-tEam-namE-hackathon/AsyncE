use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use candid::Principal;
use ic_websocket_cdk::ClientPrincipal;

use crate::{chat::Chat, group::Group, user::User, video::Video};

pub type UserStore = BTreeMap<Principal, User>;
pub type GroupStore = BTreeMap<String, Group>;
pub type VideoStore = BTreeMap<String, BTreeMap<String, Video>>;
pub type GroupInviteStore = BTreeMap<String, BTreeSet<String>>;
pub type WebSocketClientStore = BTreeSet<ClientPrincipal>;
pub type ChatStore = BTreeMap<String, BTreeMap<String, Chat>>;

thread_local! {
    pub static USERS: RefCell<UserStore> = RefCell::default();
    pub static GROUPS: RefCell<GroupStore> = RefCell::default();
    pub static VIDEOS: RefCell<VideoStore> = RefCell::default();
    pub static GROUP_INVITES: RefCell<GroupInviteStore> = RefCell::default();
    pub static WEBSOCKET_CLIENTS: RefCell<WebSocketClientStore> = RefCell::default();
    pub static CHATS: RefCell<ChatStore> = RefCell::default();
}
