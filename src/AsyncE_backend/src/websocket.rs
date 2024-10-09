use candid::CandidType;
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, ClientPrincipal, OnCloseCallbackArgs,
    OnMessageCallbackArgs, OnOpenCallbackArgs,
};
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, globals::WEBSOCKET_CLIENTS};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum WebsocketEventMessageData {
    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "group_invited")]
    GroupInvited(String),

    #[serde(rename = "add_chat")]
    AddChat(Chat),
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct WebsocketEventMessage {
    #[serde(rename = "type")]
    pub ty: String,
    pub data: WebsocketEventMessageData,
}

impl WebsocketEventMessage {
    fn candid_serialize(&self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    pub fn new_group_invited(group_id: &str) -> Self {
        Self {
            ty: String::from("group_invited"),
            data: WebsocketEventMessageData::GroupInvited(group_id.to_string()),
        }
    }

    pub fn new_ping() -> Self {
        Self {
            ty: String::from("ping"),
            data: WebsocketEventMessageData::Ping,
        }
    }

    pub fn new_chat(chat: Chat) -> Self {
        Self {
            ty: String::from("add_chat"),
            data: WebsocketEventMessageData::AddChat(chat),
        }
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
    let msg = WebsocketEventMessage::new_ping();
    send_websocket_message(args.client_principal, msg);

    WEBSOCKET_CLIENTS
        .with_borrow_mut(|websocket_clients| websocket_clients.insert(args.client_principal));
}

pub fn on_message(args: OnMessageCallbackArgs) {
    let app_msg: WebsocketEventMessage = candid::decode_one(&args.message).unwrap();
    ic_cdk::println!("Received message: {:?}", app_msg);
    // send_app_message(args.client_principal, new_msg)
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
