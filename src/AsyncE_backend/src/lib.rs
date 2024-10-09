#![allow(non_snake_case)]

pub mod globals;
pub mod group;
pub mod invite;
pub mod user;
pub mod video;
pub mod websocket;

use crate::{group::Group, user::User, video::Video, websocket::WebsocketEventMessage};
use getrandom::register_custom_getrandom;
use globals::{GROUPS, GROUP_INVITES, USERS, VIDEOS};
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, WsHandlers, WsInitParams,
};

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Ok(())
}

register_custom_getrandom!(custom_getrandom);

#[ic_cdk::init]
fn init() {
    let handlers = WsHandlers {
        on_open: Some(websocket::on_open),
        on_message: Some(websocket::on_message),
        on_close: Some(websocket::on_close),
    };

    let params = WsInitParams::new(handlers);

    ic_websocket_cdk::init(params);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let users_store = USERS.with_borrow(|users| users.clone());
    let groups_store = GROUPS.with_borrow(|groups| groups.clone());
    let videos_store = VIDEOS.with_borrow(|videos| videos.clone());
    let group_invites_store = GROUP_INVITES.with_borrow(|group_invites| group_invites.clone());

    ic_cdk::storage::stable_save((users_store, groups_store, videos_store, group_invites_store))
        .unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (user_store, group_store, videos_store, group_invites_store) =
        ic_cdk::storage::stable_restore().unwrap();
    USERS.with_borrow_mut(|users| *users = user_store);
    GROUPS.with_borrow_mut(|groups| *groups = group_store);
    VIDEOS.with_borrow_mut(|videos| *videos = videos_store);
    GROUP_INVITES.with_borrow_mut(|group_invites| *group_invites = group_invites_store);
}

ic_cdk::export_candid!();
