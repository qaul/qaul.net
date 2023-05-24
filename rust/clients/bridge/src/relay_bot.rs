use matrix_sdk::{
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    Client, ClientConfig, SyncSettings,
};
use url::Url;

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
    if let Room::Joined(room) = room {
        let msg_body = if let SyncMessageEvent {
            content:
                MessageEventContent {
                    msgtype: MessageType::Text(TextMessageEventContent { body: msg_body, .. }),
                    ..
                },
            ..
        } = event
        {
            msg_body
        } else {
            return;
        };

        if msg_body.contains("!qaul") {
            let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                "I am a message sent from qaul network\n",
            ));
            println!("sending");
            // send our message to the room we found the "!qaul command
            room.send(content, None).await.unwrap();
            println!("message sent");
        }
    }
}

async fn login (
    homeserver_url: &str,
    username: &str,
    password: &str,
) -> Result<(), matrix_sdk::Error> {
    // the location for `JsonStore` to save files to
    let mut home = dirs::home_dir().expect("no home directory found");
    home.push("qaul_bot");
    let client_config = ClientConfig::new().store_path(home);
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    // create a new Client with the given homeserver url and config
    let client = Client::new_with_config(homeserver_url, client_config).unwrap();
    client.login(&username, &password, None, Some("command bot")).await?;
    println!("logged in as {}", username);

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    client.sync_once(SyncSettings::default()).await.unwrap();
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client.register_event_handler(on_room_message).await;

    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    // this keeps state from the server streaming in to CommandBot via the
    // EventHandler trait
    client.sync(settings).await;
    Ok(())
}

#[tokio::main]
pub async fn connect()->Result<(), matrix_sdk::Error>{
    println!("Connecting to Matrix Bot");
    // TODO : Instead of passing direct arguments, Either take them from CLI or pass it secretly.
    // this is something which we don't need to be exposed.
    login("https://matrix.org","qaul-bot","qaul123promise").await?;
    Ok(())
}
