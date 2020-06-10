//! A simple chat app built on the Ratman router
//!
//! It doesn't actually implement chat logic, as that would be silly
//! (maybe another test could?), but shows how you can create messages
//! by taking some data structure, serialising it, and then addressing
//! the message to somewhere.
//!
//! As you can see the message isn't modified by the routing layer.
//! Still, you should use some mechanism to seal and sign your
//! payload.  The "Identity" used for Sender and Recipient is 32
//! bytes: the right length for a curve25519 key!

use async_std::task;
use bincode;
use netmod_mem::MemMod;
use ratman::{Identity, Message, MsgId, Recipient, Result, Router, TimePair};
use serde::{Deserialize, Serialize};

/// A message from someone
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ChatMessage {
    nick: String,
    text: String,
}

impl ChatMessage {
    fn to_msg(&self, sender: Identity, recp: Identity) -> Message {
        let payload = bincode::serialize(self).unwrap();
        let recipient = Recipient::User(recp);

        Message {
            id: MsgId::random(),
            recipient,
            sender,
            payload,
            timesig: TimePair::sending(),
            sign: vec![],
        }
    }
}

async fn build_network() -> Result<()> {
    // Build two channels in memory
    let mm1 = MemMod::new();
    let mm2_1 = MemMod::new();
    let mm2_3 = MemMod::new();
    let mm3 = MemMod::new();
    mm1.link(&mm2_1);
    mm2_3.link(&mm3);

    // Initialise three empty routers
    let r1 = Router::new();
    let r2 = Router::new();
    let r3 = Router::new();

    // Attach endpoints so the topology is r1 - r2 - r3
    r1.add_endpoint(mm1).await;
    r2.add_endpoint(mm2_1).await;
    r2.add_endpoint(mm2_3).await;
    r3.add_endpoint(mm3).await;

    // Create two users and add them to the routers
    let u1 = dbg!(Identity::random());
    r1.add_user(u1).await?;

    let u3 = dbg!(Identity::random());
    r3.add_user(u3).await?;

    // And mark them "online"
    r1.online(u1).await?;
    r3.online(u3).await?;

    // The routers will now start announcing their new users on the
    // micro-network.  You can now poll for new user discoveries.
    assert_eq!(r1.discover().await, u3);

    // We need some serialisation format. Let's use bincode
    let hello = ChatMessage {
        nick: "alice".into(),
        text: "Hey bob, how are you?".into(),
    };

    // Create a message from Alice (u1) to Bob (u3)
    let msg = hello.to_msg(u1, u3);

    r1.send(msg.clone()).await?;
    assert_eq!(r3.next().await.remove_recv_time(), msg);
    Ok(())
}

#[test]
fn very_simple_chat() {
    task::block_on(build_network()).unwrap();
}
