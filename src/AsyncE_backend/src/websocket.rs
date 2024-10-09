use candid::CandidType;
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, ClientPrincipal, OnCloseCallbackArgs,
    OnMessageCallbackArgs, OnOpenCallbackArgs,
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum WebsocketEventMessageData {
    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "group_invited")]
    GroupInvited(String),
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
    ic_cdk::println!("Client connected");
    send_app_message(
        args.client_principal,
        WebsocketEventMessage::new_group_invited("lol"),
    );
}

pub fn on_message(args: OnMessageCallbackArgs) {
    let app_msg: WebsocketEventMessage = candid::decode_one(&args.message).unwrap();
    ic_cdk::println!("Received message: {:?}", app_msg);
    // send_app_message(args.client_principal, new_msg)
}

fn send_app_message(client_principal: ClientPrincipal, msg: WebsocketEventMessage) {
    ic_cdk::println!("Sending message: {:?}", msg);

    if let Err(e) = ic_websocket_cdk::send(client_principal, msg.candid_serialize()) {
        ic_cdk::println!("Could not send message: {}", e);
    }
}

pub fn on_close(args: OnCloseCallbackArgs) {
    ic_cdk::println!("Client {} disconnected", args.client_principal);
}
