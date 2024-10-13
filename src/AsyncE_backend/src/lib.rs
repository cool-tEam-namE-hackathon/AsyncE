#![allow(non_snake_case)]

pub mod chat;
pub mod chunk;
pub mod globals;
pub mod group;
pub mod invite;
pub mod primary_key;
pub mod user;
pub mod video;
pub mod websocket;

use crate::{
    chat::Chat, group::GroupQueryResponse, video::Video, websocket::WebsocketEventMessage,
};
use globals::{CHATS, GROUPS, GROUP_INVITES, PRIMARY_KEY_CONTAINERS, USERS, VIDEOS};
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, WsHandlers, WsInitParams,
};

#[ic_cdk::init]
fn init() {
    let handlers = WsHandlers {
        on_open: Some(websocket::on_open),
        on_message: Some(websocket::on_message),
        on_close: Some(websocket::on_close),
    };

    ic_websocket_cdk::init(WsInitParams::new(handlers));
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let users_store = USERS.with_borrow(|users| users.clone());
    let groups_store = GROUPS.with_borrow(|groups| groups.clone());
    let videos_store = VIDEOS.with_borrow(|videos| videos.clone());
    let group_invites_store = GROUP_INVITES.with_borrow(|group_invites| group_invites.clone());
    let chat_store = CHATS.with_borrow(|chats| chats.clone());
    let primary_key_store =
        PRIMARY_KEY_CONTAINERS.with_borrow(|primary_key_containers| primary_key_containers.clone());

    ic_cdk::storage::stable_save((
        users_store,
        groups_store,
        videos_store,
        group_invites_store,
        chat_store,
        primary_key_store,
    ))
    .unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (
        user_store,
        group_store,
        videos_store,
        group_invites_store,
        chat_store,
        primary_key_containers_store,
    ) = ic_cdk::storage::stable_restore().unwrap();

    USERS.with_borrow_mut(|users| *users = user_store);
    GROUPS.with_borrow_mut(|groups| *groups = group_store);
    VIDEOS.with_borrow_mut(|videos| *videos = videos_store);
    GROUP_INVITES.with_borrow_mut(|group_invites| *group_invites = group_invites_store);
    CHATS.with_borrow_mut(|chats| *chats = chat_store);
    PRIMARY_KEY_CONTAINERS.with_borrow_mut(|primary_key_containers| {
        *primary_key_containers = primary_key_containers_store
    });
}

ic_cdk::export_candid!();
