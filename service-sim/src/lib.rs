#![cfg(test)]
use libqaul::{Qaul, QaulResult, UserAuth, UserData};
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

fn get_system_and_auth() -> (Qaul, UserAuth) {
    let mut qaul = Qaul::start();
    let auth = qaul
        .user_create("password")
        .expect("Could not create test user.");
    return (qaul, auth);
}

#[test]
fn update_user_updates_applied_in_order() {
    use QaulApiEvent::*;
    let (mut qaul, auth) = get_system_and_auth();

    let data1 = UserData::new().with_real_name("Danny Default");
    let data2 = UserData::new().with_real_name("Dougie D'Ifferent");

    let qaul = new_fallible_engine(resolve)
        .queue_events(&[
            UpdateUser {
                user: auth.clone(),
                data: data1,
            },
            UpdateUser {
                user: auth.clone(),
                data: data2,
            },
        ])
        .resolve_in_order(qaul)
        .expect("Resolution of events failed. Error");

    let user = qaul
        .user_get(auth.clone())
        .expect("Could not get test user.");
    assert_eq!(user.data.real_name, Some(String::from("Dougie D'Ifferent")));
}
