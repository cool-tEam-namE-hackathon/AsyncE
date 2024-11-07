#![allow(non_snake_case)]

pub mod chat;
pub mod chunk;
pub mod globals;
pub mod group;
pub mod http;
pub mod invite;
pub mod meeting;
pub mod primary_key;
pub mod user;
pub mod websocket;

use crate::{
    chat::Chat,
    group::{GroupMemberRole, GroupQueryResponse},
    invite::GroupInviteResponse,
    meeting::MeetingHeader,
    user::UserCredentialsResponse,
    websocket::WebsocketEventMessage,
};
use globals::{CHATS, GROUPS, GROUP_INVITES, MEETINGS, PRIMARY_KEY_CONTAINERS, USERS};
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
    http::poll_concat_requests();
    http::poll_subtitle_requests();
    user::poll_user_subscriptions();
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let users_store = USERS.with_borrow(|users| users.clone());
    let groups_store = GROUPS.with_borrow(|groups| groups.clone());
    let meetings_store = MEETINGS.lock().unwrap().clone();
    let group_invites_store = GROUP_INVITES.with_borrow(|group_invites| group_invites.clone());
    let chat_store = CHATS.with_borrow(|chats| chats.clone());
    let primary_key_store =
        PRIMARY_KEY_CONTAINERS.with_borrow(|primary_key_containers| primary_key_containers.clone());

    ic_cdk::storage::stable_save((
        users_store,
        groups_store,
        meetings_store,
        group_invites_store,
        chat_store,
        primary_key_store,
    ))
    .expect("FAILED TO STABLE SAVE DATA!");
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (
        user_store,
        group_store,
        meetings_store,
        group_invites_store,
        chat_store,
        primary_key_containers_store,
    ) = ic_cdk::storage::stable_restore().expect("FAILED TO STABLE RESTORE DATA!");

    USERS.with_borrow_mut(|users| *users = user_store);
    GROUPS.with_borrow_mut(|groups| *groups = group_store);
    *MEETINGS.lock().unwrap() = meetings_store;
    GROUP_INVITES.with_borrow_mut(|group_invites| *group_invites = group_invites_store);
    CHATS.with_borrow_mut(|chats| *chats = chat_store);
    PRIMARY_KEY_CONTAINERS.with_borrow_mut(|primary_key_containers| {
        *primary_key_containers = primary_key_containers_store
    });

    http::poll_concat_requests();
    http::poll_subtitle_requests();
    user::poll_user_subscriptions();

    // init_rng()
}

// thread_local! {
//     static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
// }

// register_custom_getrandom!(custom_getrandom);

// fn init_rng() {
//     ic_cdk_timers::set_timer(Duration::ZERO, || {
//         ic_cdk::spawn(async {
//             let (seed,) = ic_cdk::api::management_canister::main::raw_rand()
//                 .await
//                 .unwrap();

//             RNG.with(|rng| *rng.borrow_mut() = Some(StdRng::from_seed(seed.try_into().unwrap())));
//         })
//     });
// }

// fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
//     RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
//     Ok(())
// }

ic_cdk::export_candid!();
