#![cfg(test)]
use libqaul::{Identity, Qaul, QaulResult, UserAuth, UserData};
use visn::{new_fallible_engine, KnowledgeEngine};

#[derive(Clone)]
enum QaulApiEvent {
    UpdateUser { user: UserAuth, data: UserData },
    GetUser { user: UserAuth },
    DeleteUser { user: UserAuth },
}

fn resolve(event: QaulApiEvent, system: Qaul) -> QaulResult<Qaul> {
    use QaulApiEvent::*;
    match event {
        UpdateUser { user, data } => system.user_update(user, data)?,
        DeleteUser { user } => system.user_delete(user)?,
        _ => unimplemented!(),
    }
    Ok(system)
}

fn test_auth() -> UserAuth {
    let id: Identity = [00; 12].into();
    let k = String::new();
    UserAuth::Trusted(id, k)
}

fn system_with_auth() -> Qaul {
    let mut qaul = Qaul::start();
    qaul.user_inject(test_auth())
        .expect("Could not create test user.");
    qaul
}

#[test]
fn update_user_updates_applied_in_order() {
    use QaulApiEvent::*;
    let auth = test_auth();

    let update1 = UserData::new().with_real_name("Danny Default");
    let update2 = UserData::new().with_real_name("Dougie D'Ifferent");

    let qaul = new_fallible_engine(resolve)
        .queue_events(&[
            UpdateUser {
                user: auth.clone(),
                data: update1,
            },
            UpdateUser {
                user: auth.clone(),
                data: update2,
            },
        ])
        .resolve_in_order(system_with_auth)
        .expect("Resolution of events failed. Error");

    let user = qaul
        .user_get(auth.clone())
        .expect("Could not get test user.");
    assert_eq!(user.data.real_name, Some(String::from("Dougie D'Ifferent")));
}
