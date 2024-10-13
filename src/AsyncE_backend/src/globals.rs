use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use candid::Principal;
use ic_websocket_cdk::ClientPrincipal;

use crate::{chat::Chat, group::Group, primary_key::PrimaryKeyContainer, user::User, video::Video};

pub type UserStore = BTreeMap<Principal, User>;
pub type GroupStore = BTreeMap<u128, Group>;
pub type VideoStore = BTreeMap<u128, BTreeMap<u128, Video>>;
pub type GroupInviteStore = BTreeMap<String, BTreeSet<u128>>;
pub type WebSocketClientStore = BTreeSet<ClientPrincipal>;
pub type ChatStore = BTreeMap<u128, BTreeMap<u128, Chat>>;
pub type PrimaryKeyContainerStore = PrimaryKeyContainer;
pub type VideoUploadStore = BTreeMap<u128, Vec<u8>>;

thread_local! {
    pub static USERS: RefCell<UserStore> = RefCell::default();
    pub static GROUPS: RefCell<GroupStore> = RefCell::default();
    pub static VIDEOS: RefCell<VideoStore> = RefCell::default();
    pub static GROUP_INVITES: RefCell<GroupInviteStore> = RefCell::default();
    pub static WEBSOCKET_CLIENTS: RefCell<WebSocketClientStore> = RefCell::default();
    pub static CHATS: RefCell<ChatStore> = RefCell::default();
    pub static PRIMARY_KEY_CONTAINERS: RefCell<PrimaryKeyContainer> = RefCell::default();
    pub static VIDEO_UPLOADS: RefCell<VideoUploadStore> = RefCell::default();
}
