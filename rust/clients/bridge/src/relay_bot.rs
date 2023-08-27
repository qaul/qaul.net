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
    media::{MediaFormat, MediaRequest, MediaType},
    room::Room,
    ruma::{
        events::{
            room::{
                member::MemberEventContent,
                message::{
                    FileMessageEventContent, ImageMessageEventContent, MessageEventContent,
                    MessageType, TextMessageEventContent,
                },
            },
            AnyMessageEventContent, StrippedStateEvent, SyncMessageEvent,
        },
        RoomId,
    },
    Client, ClientConfig, SyncSettings,
};
use prost::Message;
// use std::fs::File;
use std::io::prelude::*;
use tokio::time::{sleep, Duration};
use url::Url;
use uuid::Uuid;
// static CONFIG: Storage<RwLock<Configuration>> = Storage::new();
/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chat.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
}
use crate::{chat, chatfile, configuration::MatrixRoom, group, users};

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
        match &event {
            SyncMessageEvent {
                content: MessageEventContent { msgtype, .. },
                sender: msg_sender,
                ..
            } => {
                match msgtype {
                    MessageType::Audio(_) => todo!(),
                    MessageType::Emote(_) => todo!(),
                    MessageType::File(FileMessageEventContent {
                        body: file_name,
                        url: file_url,
                        ..
                    }) => {
                        let request = MediaRequest {
                            format: MediaFormat::File,
                            media_type: MediaType::Uri(file_url.as_ref().unwrap().clone()),
                        };
                        let client = MATRIX_CLIENT.get();
                        let file_bytes = client.get_media_content(&request, true).await.unwrap();
                        let path_string = Storage::get_path();
                        let path = Path::new(path_string.as_str());
                        let output_file_path = path.join(file_name);
                        let mut file = std::fs::File::create(output_file_path).unwrap();
                        let _ = file.write_all(&file_bytes);
                        println!("File Saved Successfully");
                        send_file_to_qaul(
                            room.room_id(),
                            file_name,
                            format!("{} by {}", file_name, msg_sender),
                        );
                    }
                    MessageType::Image(ImageMessageEventContent {
                        body: file_name,
                        url: image_url,
                        ..
                    }) => {
                        let request = MediaRequest {
                            format: MediaFormat::File,
                            media_type: MediaType::Uri(image_url.as_ref().unwrap().clone()),
                        };
                        let client = MATRIX_CLIENT.get();
                        let file_bytes = client.get_media_content(&request, true).await.unwrap();
                        let path_string = Storage::get_path();
                        let path = Path::new(path_string.as_str());
                        let output_file_path = path.join(file_name);
                        let mut file = std::fs::File::create(output_file_path).unwrap();
                        let _ = file.write_all(&file_bytes);
                        println!("File Saved Successfully");
                        send_file_to_qaul(
                            room.room_id(),
                            file_name,
                            format!("{} by {}", file_name, msg_sender),
                        );
                    }
                    MessageType::Location(_) => todo!(),
                    MessageType::Notice(_) => todo!(),
                    MessageType::ServerNotice(_) => todo!(),
                    MessageType::Text(TextMessageEventContent { body: msg_body, .. }) => {
                        if msg_sender != "@qaul-bot:matrix.org" {
                            let msg_text = format!("{} : {}", msg_sender, msg_body);
                            // Check first whether from feed room or what.
                            send_qaul(msg_text, room.room_id());

                            // on receiving !qaul from matrix, Send message
                            if msg_body.contains("!qaul") {
                                let content = AnyMessageEventContent::RoomMessage(
                                    MessageEventContent::text_plain(
                                        "I am a message sent from qaul network\n",
                                    ),
                                );
                                room.send(content, None).await.unwrap();
                            }

                            // on receiving !help from matrix, Give brief of all possible commands.
                            if msg_body.contains("!help") {
                                let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                                    "!qaul : Ping to check if the bot is active or not.\n!users : Get list of all the users on the network.\n!invite {qaul_user_id} : To invite a user from the qaul into this matrix room.\n !remove {qaul_user_id} : To remove a user from the qaul into this matrix room.\n!group-info : Get details for the qaul group with which this matrix room is connected.",
                                ));
                                room.send(content, None).await.unwrap();
                            }

                            // on receiving !qaul in matrix, Send message
                            if msg_body.contains("!invite") {
                                // TODO : Do code cleanup just like !remove command so we handle all core logics in qaul world without disturbance to relay bot.
                                let matrix_user =
                                    room.get_member(&msg_sender).await.unwrap().unwrap();
                                // Admin Powers
                                if matrix_user.power_level() == 100 {
                                    let mut iter = msg_body.split_whitespace();
                                    let _command = iter.next().unwrap();
                                    // TODO : Try to return an error if userID is wrong.
                                    let qaul_user_id = iter.next().unwrap().to_string();
                                    // creating new group with request_id as matrix room name.
                                    // request ID = sender + room_name + qaul_user_id
                                    let room_id_string = room.room_id().to_string();
                                    let sender_string = msg_sender.to_string();
                                    let request_id = format!(
                                        "invite#{}#{}#{}",
                                        room_id_string, sender_string, qaul_user_id
                                    );
                                    println!("{}", request_id);
                                    // Create group only if the mapping between a qaul grp and matrix room doesn't exist.
                                    // If it exist then please check if user already exist or not. If not then invite :)
                                    let config = MATRIX_CONFIG.get().write().unwrap().clone();
                                    let room_id = room.room_id();
                                    let qaul_group_id: Option<Uuid> = find_key_for_value(
                                        config.room_map.clone(),
                                        room_id.clone(),
                                    );
                                    if qaul_group_id == None {
                                        group::Group::create_group(
                                            format!("{}", msg_sender.to_owned()).to_owned(),
                                            request_id,
                                        );
                                        // Acknowledge about sent invitation to qaul user.
                                        let content = AnyMessageEventContent::RoomMessage(
                                        MessageEventContent::text_plain("User has been invited. Please wait until user accepts the invitation."),
                                    );
                                        room.send(content, None).await.unwrap();
                                    } else {
                                        // Get the list of users who are members to the given room.
                                        group::Group::group_info(
                                            chat::Chat::uuid_string_to_bin(
                                                qaul_group_id.unwrap().to_string(),
                                            )
                                            .unwrap(),
                                            request_id,
                                        );
                                        println!("The Room Mapping already exist for this room");
                                        // Else Invite the given user in same mapping of the matrix room.
                                    }
                                } else {
                                    // Not Admin
                                    let content = AnyMessageEventContent::RoomMessage(
                                        MessageEventContent::text_plain(
                                            "Only Admins can perform this operation.",
                                        ),
                                    );
                                    room.send(content, None).await.unwrap();
                                }
                            }

                            // on receiving !users-list in matrix, Send it to command line
                            if msg_body.contains("!users") {
                                users::Users::request_user_list(room.room_id().to_string());
                            }

                            // remove the people from the matrix room
                            if msg_body.contains("!remove") {
                                let matrix_user =
                                    room.get_member(&msg_sender).await.unwrap().unwrap();
                                // Admin Powers
                                if matrix_user.power_level() == 100 {
                                    let mut iter = msg_body.split_whitespace();
                                    let _command = iter.next().unwrap();
                                    // TODO : Try to return an error if userID is wrong.
                                    let qaul_user_id = iter.next().unwrap().to_string();
                                    let room_id_string = room.room_id().to_string();
                                    let sender_string = msg_sender.to_string();
                                    let request_id = format!(
                                        "remove#{}#{}#{}",
                                        room_id_string, sender_string, qaul_user_id
                                    );
                                    println!("{}", request_id);
                                    // Create group only if the mapping between a qaul grp and matrix room doesn't exist.
                                    // If it exist then please check if user already exist or not. If not then invite :)
                                    let config = MATRIX_CONFIG.get().write().unwrap().clone();
                                    let room_id = room.room_id();
                                    let qaul_group_id: Option<Uuid> = find_key_for_value(
                                        config.room_map.clone(),
                                        room_id.clone(),
                                    );
                                    if qaul_group_id == None {
                                        // No room mapping exist
                                        let content =
                                            AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                                                "No qaul group is mapped to this Matrix room. Please invite qaul users to this room.",
                                            ));
                                        room.send(content, None).await.unwrap();
                                    } else {
                                        group::Group::group_info(
                                            chat::Chat::uuid_string_to_bin(
                                                qaul_group_id.unwrap().to_string(),
                                            )
                                            .unwrap(),
                                            request_id,
                                        );
                                    }
                                } else {
                                    // Not Admin
                                    let content = AnyMessageEventContent::RoomMessage(
                                        MessageEventContent::text_plain(
                                            "Only Admins can perform this operation.",
                                        ),
                                    );
                                    room.send(content, None).await.unwrap();
                                }
                            }

                            // on receiving !qaul-info in matrix, You get the details of the group information.
                            if msg_body.contains("!group-info") {
                                let config = MATRIX_CONFIG.get().write().unwrap().clone();
                                let room_id = room.room_id();
                                let qaul_group_id: Option<Uuid> =
                                    find_key_for_value(config.room_map.clone(), room_id.clone());
                                if qaul_group_id == None {
                                    // No room mapping exist
                                    let content =
                                   AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                                       "No qaul group is mapped to this Matrix room. Please invite qaul users to this room.",
                                   ));
                                    room.send(content, None).await.unwrap();
                                } else {
                                    let request_id = format!("info#{}#_#_", room_id).to_string();
                                    group::Group::group_info(
                                        chat::Chat::uuid_string_to_bin(
                                            qaul_group_id.unwrap().to_string(),
                                        )
                                        .unwrap(),
                                        request_id,
                                    );
                                }
                            }
                        } else {
                            println!("Sent the message in the matrix room by !qaul-bot");
                        }
                    }
                    MessageType::Video(_) => todo!(),
                    MessageType::VerificationRequest(_) => todo!(),
                    _ => todo!(),
                };
            }
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
pub async fn connect(arguments: HashMap<usize, String>) -> Result<(), matrix_sdk::Error> {
    println!("Connecting to Matrix Bot");

    // Configuration for starting of the bot
    let path_string = Storage::get_path();
    let path = Path::new(path_string.as_str());
    let config_path = path.join("matrix.yaml");
    let mut config: MatrixConfiguration = match Config::builder()
        .add_source(File::with_name(&config_path.to_str().unwrap()))
        .build()
    {
        Err(_) => MatrixConfiguration::default(),
        Ok(c) => c.try_deserialize::<MatrixConfiguration>().unwrap(),
    };
    MatrixConfiguration::save(config.clone());
    if config.relay_bot.homeserver == "" {
        config.relay_bot.homeserver = arguments.get(&1).unwrap().clone();
    }
    if config.relay_bot.bot_id == "" {
        config.relay_bot.bot_id = arguments.get(&2).unwrap().clone();
    }
    if config.relay_bot.bot_password == "" {
        config.relay_bot.bot_password = arguments.get(&3).unwrap().clone();
    }
    MatrixConfiguration::save(config.clone());
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
    println!("Message from Matrix arrived");
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

fn send_file_to_qaul(room_id: &RoomId, file_name: &String, description: String) {
    println!("File from Matrix arrived in Qaul");
    let mut config = MATRIX_CONFIG.get().write().unwrap();
    // config.room_map; Find the key corresponsing to given value and use feed to send msg to the mapped gropID.
    // forward the message in that qaul group instead of feed.
    let qaul_id = find_key_for_value(config.room_map.clone(), room_id.clone());
    if qaul_id.is_some() {
        chatfile::ChatFile::send_file(
            chat::Chat::uuid_string_to_bin(qaul_id.unwrap().to_string()).unwrap(),
            file_name.clone(),
            description,
        );
        if let Some(qaul_room) = config.room_map.get_mut(&qaul_id.unwrap()) {
            qaul_room.last_index += 1;
        }
    } else {
        println!("Not Possible to send file into feed");
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
