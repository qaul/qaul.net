// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Relay Bot functions
//!
//! Logging in and listening for the message on matrix room and sending messages from qaul.
use super::rpc::Rpc;
use matrix_sdk::{
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    Client, ClientConfig, SyncSettings,
};
use prost::Message;
use url::Url;
/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
}
use crate::cli::Cli;
enum EventType {
    Cli(String),
}

// Setup a storage object for the Client to make it available globally
pub static MATRIX_CLIENT: state::Storage<Client> = state::Storage::new();

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
    if let Room::Joined(room) = room {
        let (msg_body, msg_sender) = if let SyncMessageEvent {
            content:
                MessageEventContent {
                    msgtype: MessageType::Text(TextMessageEventContent { body: msg_body, .. }),
                    ..
                },
            sender: msg_sender,
            ..
        } = event
        {
            (msg_body, msg_sender)
        } else {
            return;
        };
        if msg_sender != "@qaul-bot:matrix.org" {
            let msg_text = format!("{} : {}", msg_sender, msg_body);
            let proto_message = proto::Feed {
                message: Some(proto::feed::Message::Send(proto::SendMessage {
                    content: msg_text,
                })),
            };

            // encode message
            let mut buf = Vec::with_capacity(proto_message.encoded_len());
            proto_message
                .encode(&mut buf)
                .expect("Vec<u8> provides capacity as needed");

            // send message to the qaul feed
            Rpc::send_message(buf, super::rpc::proto::Modules::Feed.into(), "".to_string());

            // on receiving !qaul in matrix, Send message
            if msg_body.contains("!qaul") {
                let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                    "I am a message sent from qaul network\n",
                ));
                room.send(content, None).await.unwrap();
            }

            // on receiving !users-list in matrix, Send it to command line
            if msg_body.contains("!users-list") {
                let input_line = "users list".to_string();
                let evt = Some(EventType::Cli(input_line));
                if let Some(event) = evt {
                    match event {
                        EventType::Cli(line) => {
                            Cli::process_command(line);
                        }
                    }
                }
            }
        } else {
            println!("Sent the message in the matrix room by !qaul-bot");
        }
    }
}

async fn login(
    homeserver_url: &str,
    username: &str,
    password: &str,
) -> Result<(), matrix_sdk::Error> {
    // the location for `JsonStore` to save files to
    let mut home = dirs::config_dir().expect("no home directory found");
    home.push("qaul/matrix");
    println!("{:?}", home);
    let client_config = ClientConfig::new().store_path(home);
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");

    // create a new Client with the given homeserver url and config
    let client = Client::new_with_config(homeserver_url, client_config).unwrap();
    client
        .login(&username, &password, None, Some("command bot"))
        .await?;
    println!("logged in as {}", username);

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    client.sync_once(SyncSettings::default()).await.unwrap();
    
    // initial sync to avoid responding to messages before the bot was running.
    client.register_event_handler(on_room_message).await;

    // Store matrix client inside storage stack.
    MATRIX_CLIENT.set(client.clone());
  
    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    // this keeps state from the server streaming in to CommandBot via the
    // EventHandler trait
    client.sync(settings).await;
    Ok(())
}

#[tokio::main]
pub async fn connect() -> Result<(), matrix_sdk::Error> {
    println!("Connecting to Matrix Bot");
    // TODO : Instead of passing direct arguments, Either take them from CLI or pass it secretly.
    // this is something which we don't need to be exposed.
    login("https://matrix.org", "qaul-bot", "qaul123promise").await?;
    Ok(())
}
