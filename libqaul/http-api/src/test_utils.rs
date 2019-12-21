use std::sync::Arc;
use {
    libqaul::{users::UserAuth, Qaul, messages::Recipient},
    netmod_mem::MemMod,
    ratman::{netmod::Endpoint, Router},
};

pub struct TestNetwork {
    pub a: Arc<Qaul>,
    pub b: Arc<Qaul>,
    middle: Arc<Qaul>,
}

impl TestNetwork {
    pub fn new() -> TestNetwork {
        let mut mm1 = MemMod::new();
        let mut mm2 = MemMod::new();
        let mut mm3 = MemMod::new();
        let mut mm4 = MemMod::new();

        mm1.link(&mut mm2);
        mm3.link(&mut mm4);

        let r1 = Router::new();
        let r2 = Router::new();
        let r3 = Router::new();

        r1.modify().add_ep(mm1);
        r2.modify().add_ep(mm2);
        r2.modify().add_ep(mm3);
        r3.modify().add_ep(mm4);

        let q1 = Qaul::new(r1);
        let q2 = Qaul::new(r2);
        let q3 = Qaul::new(r3);

        TestNetwork {
            a: q1, 
            b: q3,
            middle: q2,
        }
    }

    pub fn add_user_a(&self, password: &str) -> UserAuth {
        let ua = self.a.users().create(password).expect("create user a");
        #[allow(deprecated)]
        {
            self.middle.router().discover(ua.0, 0);
            self.b.router().discover(ua.0, 0);
        }
        ua
    }

    pub fn add_user_b(&self, password: &str) -> UserAuth {
        let ua = self.b.users().create(password).expect("create user a");
        #[allow(deprecated)]
        {
            self.middle.router().discover(ua.0, 1);
            self.a.router().discover(ua.0, 0);
        }
        ua
    }
}

#[test]
fn send_and_recv() {
    let network = TestNetwork::new();
    let u1 = network.add_user_a("test");
    let u2 = network.add_user_b("test");

    network.a.services().register("test").unwrap();
    network.b.services().register("test").unwrap();

    let id = network.a.messages().send(
        u1.clone(), 
        Recipient::User(u2.0.clone()),
        "test",
        b"hewwo".to_vec()
    ).unwrap();

    #[allow(deprecated)]
    std::thread::sleep_ms(500);

    let recv = network.b.messages().poll(u2.clone(), "test").unwrap();
    assert_eq!(recv.id, id);
    assert_eq!(recv.sender, u1.0);
    assert_eq!(recv.recipient, Recipient::User(u2.0.clone()));
    assert_eq!(recv.associator, "test");
    assert_eq!(recv.payload, b"hewwo");
}
