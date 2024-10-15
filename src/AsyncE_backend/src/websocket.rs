use candid::{CandidType, Principal};
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, ClientPrincipal, OnCloseCallbackArgs,
    OnMessageCallbackArgs, OnOpenCallbackArgs,
};
use serde::{Deserialize, Serialize};

use crate::{
    chat::Chat,
    globals::{CHATS, GROUPS, USERS, WEBSOCKET_CLIENTS},
    invite::GroupInviteResponse,
    primary_key::{self, PrimaryKeyType},
    user,
};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum WebsocketEventMessage {
    Ping,
    GroupInvited(GroupInviteResponse),
    AddChat(Chat),
}

impl WebsocketEventMessage {
    fn candid_serialize(&self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }
}

#[ic_cdk::update]
fn ws_open(args: CanisterWsOpenArguments) -> CanisterWsOpenResult {
    ic_websocket_cdk::ws_open(args)
}

#[ic_cdk::update]
fn ws_close(args: CanisterWsCloseArguments) -> CanisterWsCloseResult {
    ic_websocket_cdk::ws_close(args)
}

#[ic_cdk::update]
fn ws_message(
    args: CanisterWsMessageArguments,
    msg_type: Option<WebsocketEventMessage>,
) -> CanisterWsMessageResult {
    ic_websocket_cdk::ws_message(args, msg_type)
}

#[ic_cdk::query]
fn ws_get_messages(args: CanisterWsGetMessagesArguments) -> CanisterWsGetMessagesResult {
    ic_websocket_cdk::ws_get_messages(args)
}

pub fn on_open(args: OnOpenCallbackArgs) {
    send_websocket_message(args.client_principal, WebsocketEventMessage::Ping);

    WEBSOCKET_CLIENTS
        .with_borrow_mut(|websocket_clients| websocket_clients.insert(args.client_principal));
}

pub fn on_message(args: OnMessageCallbackArgs) {
    let app_msg: WebsocketEventMessage = candid::decode_one(&args.message).unwrap();
    ic_cdk::println!("Received message: {:?}", app_msg);

    user::assert_user_logged_in_from(args.client_principal).unwrap();

    match app_msg {
        WebsocketEventMessage::Ping => {}

        WebsocketEventMessage::GroupInvited { .. } => {}

        WebsocketEventMessage::AddChat(mut chat) => {
            let name = USERS
                .with_borrow(|users| {
                    users
                        .get(&args.client_principal)
                        .map(|x| x.username.clone())
                })
                .unwrap();

            GROUPS.with_borrow(|groups| {
                let group = groups
                    .get(&chat.group_id)
                    .expect("Cannot find group with this ID!");
                if group.owner != name && !group.users.contains(&name) {
                    panic!("This user is not in this group!")
                }

                chat.id = primary_key::get_primary_key(PrimaryKeyType::Chat);
                chat.username = name;
                chat.created_time_unix = ic_cdk::api::time() as u128;

                CHATS.with_borrow_mut(|chats| {
                    chats
                        .entry(chat.group_id)
                        .or_default()
                        .insert(chat.id, chat.clone());

                    broadcast_websocket_message(WebsocketEventMessage::AddChat(chat));
                })
            });
        }
    }
}

pub fn broadcast_websocket_message(msg: WebsocketEventMessage) {
    WEBSOCKET_CLIENTS.with_borrow(|websocket_clients| {
        for &client_principal in websocket_clients.iter() {
            send_websocket_message(client_principal, msg.clone());
        }
    })
}

pub fn send_websocket_message(client_principal: ClientPrincipal, msg: WebsocketEventMessage) {
    ic_cdk::println!("Sending message to {}: {:?}", client_principal, msg);

    if let Err(e) = ic_websocket_cdk::send(client_principal, msg.candid_serialize()) {
        ic_cdk::println!(
            "Could not send message to {} with payload: {:?}: {}",
            client_principal,
            msg,
            e
        );
    }
}

pub fn on_close(args: OnCloseCallbackArgs) {
    ic_cdk::println!("Client {} disconnected", args.client_principal);

    WEBSOCKET_CLIENTS
        .with_borrow_mut(|websocket_clients| websocket_clients.remove(&args.client_principal));
}

pub fn send_group_invited_notif(principal: Principal, group_id: u128, group_name: &str) {
    send_websocket_message(
        principal,
        WebsocketEventMessage::GroupInvited(GroupInviteResponse {
            group_id,
            group_name: group_name.to_string(),
        }),
    );
}
