use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use candid::Principal;
use ic_websocket_cdk::ClientPrincipal;
use parking_lot::FairMutex;

use crate::{
    chat::Chat, group::Group, meeting::Meeting, primary_key::PrimaryKeyContainer, user::User,
};

pub type UserStore = BTreeMap<Principal, User>;
pub type GroupStore = BTreeMap<u128, Group>;
pub type MeetingStore = BTreeMap<u128, BTreeMap<u128, Meeting>>;
pub type GroupInviteStore = BTreeMap<String, BTreeSet<u128>>;
pub type WebSocketClientStore = BTreeSet<ClientPrincipal>;
pub type ChatStore = BTreeMap<u128, BTreeMap<u128, Chat>>;
pub type PrimaryKeyContainerStore = PrimaryKeyContainer;
pub type VideoUploadStore = BTreeMap<String, Vec<u8>>;

thread_local! {
    pub static USERS: RefCell<UserStore> = RefCell::default();
    pub static GROUPS: RefCell<GroupStore> = RefCell::default();
    pub static GROUP_INVITES: RefCell<GroupInviteStore> = RefCell::default();
    pub static WEBSOCKET_CLIENTS: RefCell<WebSocketClientStore> = RefCell::default();
    pub static CHATS: RefCell<ChatStore> = RefCell::default();
    pub static PRIMARY_KEY_CONTAINERS: RefCell<PrimaryKeyContainer> = RefCell::default();
    pub static VIDEO_UPLOADS: RefCell<VideoUploadStore> = RefCell::default();
}

lazy_static::lazy_static! {
    pub static ref MEETINGS: FairMutex<MeetingStore> = FairMutex::default();
}
