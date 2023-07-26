// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Relay Bot functions
//!
//! Logging in and listening for the message on matrix room and sending messages from qaul.
use super::configuration::MatrixConfiguration;
use std::{collections::HashMap, path::Path, sync::RwLock};
// use state::Storage;
use super::rpc::Rpc;
use config::*;
use libqaul::storage::Storage;
use matrix_sdk::{
    room::Room,
    ruma::{
        events::{
            room::{
                member::MemberEventContent,
                message::{MessageEventContent, MessageType, TextMessageEventContent},
            },
            AnyMessageEventContent, StrippedStateEvent, SyncMessageEvent,
        },
        RoomId,
    },
    Client, ClientConfig, SyncSettings,
};
use prost::Message;
use tokio::time::{sleep, Duration};
use url::Url;
use uuid::{fmt, Uuid};
// static CONFIG: Storage<RwLock<Configuration>> = Storage::new();
/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chat.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
}
use crate::{chat, cli::Cli, configuration::MatrixRoom, group, users};
enum EventType {
    Cli(String),
}

// Setup a storage object for the Client to make it available globally
pub static MATRIX_CLIENT: state::Storage<Client> = state::Storage::new();
pub static MATRIX_CONFIG: state::Storage<RwLock<MatrixConfiguration>> = state::Storage::new();

async fn on_stripped_state_room(
    room_member: StrippedStateEvent<MemberEventContent>,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().await.unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.accept_invitation().await {
            // retry autojoin due to synapse sending invites, before the
            // invited user can join for more information see
            // https://github.com/matrix-org/synapse/issues/4345
            eprintln!(
                "Failed to join room {} ({:?}), retrying in {}s",
                room.room_id(),
                err,
                delay
            );

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                eprintln!("Can't join room {} ({:?})", room.room_id(), err);
                break;
            }
        }
        println!("Successfully joined room {}", room.room_id());
    }
}

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
            send_qaul(msg_text, room.room_id());

            // on receiving !qaul in matrix, Send message
            if msg_body.contains("!qaul") {
                let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                    "I am a message sent from qaul network\n",
                ));
                room.send(content, None).await.unwrap();
            }

            // on receiving !qaul in matrix, Send message
            if msg_body.contains("!invite") {
                let mut iter = msg_body.split_whitespace();
                let _command = iter.next().unwrap();
                // Try to return an error if userID is wrong.
                let qaul_user_id = iter.next().unwrap().to_string();
                // creating new group with request_id as matrix room name.
                // request ID = sender + room_name.
                let room_id_string = room.room_id().to_string();
                let sender_string = msg_sender.to_string();
                let request_id = format!("{}#{}#{}", room_id_string, sender_string, qaul_user_id);
                println!("{}", request_id);
                group::Group::create_group(
                    format!("{}", msg_sender.to_owned()).to_owned(),
                    request_id,
                );
            }

            // on receiving !users-list in matrix, Send it to command line
            if msg_body.contains("!users-list") {
                users::Users::request_user_list(room.room_id().to_string());
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
    client.register_event_handler(on_stripped_state_room).await;
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

    // Configuration for starting of the bot
    let path_string = Storage::get_path();
    let path = Path::new(path_string.as_str());
    let config_path = path.join("matrix.yaml");
    let config: MatrixConfiguration = match Config::builder()
        .add_source(File::with_name(&config_path.to_str().unwrap()))
        .build()
    {
        Err(e) => {
            log::error!("{}", e);
            MatrixConfiguration::default()
        }
        Ok(c) => c.try_deserialize::<MatrixConfiguration>().unwrap(),
    };

    MATRIX_CONFIG.set(RwLock::new(config.clone()));
    login(
        &config.relay_bot.homeserver,
        &config.relay_bot.bot_id,
        &config.relay_bot.bot_password,
    )
    .await?;
    Ok(())
}

fn send_qaul(msg_text: String, room_id: &RoomId) {
    let mut config = MATRIX_CONFIG.get().write().unwrap();
    // config.room_map; Find the key corresponsing to given value and use feed to send msg to the mapped gropID.
    // forward the message in that qaul group instead of feed.
    let qaul_id = find_key_for_value(config.room_map.clone(), room_id.clone());
    if qaul_id.is_some() {
        // create group send message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::Send(proto::ChatMessageSend {
                group_id: chat::Chat::uuid_string_to_bin(qaul_id.unwrap().to_string()).unwrap(),
                content: msg_text,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Chat.into(), "".to_string());
        if let Some(qaul_room) = config.room_map.get_mut(&qaul_id.unwrap()) {
            qaul_room.last_index += 1;
        }
    } else {
        // send to feed from matrix
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
        Rpc::send_message(buf, super::rpc::proto::Modules::Feed.into(), "".to_string());
        let last_index = config.feed.last_index;
        config.feed.last_index = last_index + 1;
    }

    MatrixConfiguration::save(config.clone());
}

fn find_key_for_value(map: HashMap<Uuid, MatrixRoom>, value: RoomId) -> Option<Uuid> {
    map.iter()
        .find_map(|(key, &ref val)| {
            if val.matrix_room_id == value {
                Some(key)
            } else {
                None
            }
        })
        .copied()
}
