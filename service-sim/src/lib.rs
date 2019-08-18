#![cfg(test)]
use libqaul::{Identity, Qaul, QaulResult, User, UserAuth, UserData, UserUpdate};
use visn::{new_fallible_engine, KnowledgeEngine};

#[derive(Clone)]
enum QaulApiEvent {
    UpdateUser { user: UserAuth, data: UserUpdate },
    DeleteUser { user: UserAuth },
    AddContact { user: UserAuth, contact: User },
}

fn resolve(event: QaulApiEvent, system: Qaul) -> QaulResult<Qaul> {
    use QaulApiEvent::*;
    match event {
        UpdateUser { user, data } => {
            system.user_update(user, data)?;
        }
        DeleteUser { user } => {
            system.user_delete(user)?;
        }
        AddContact { user, contact } => {
            system.contacts_add(user, contact)?;
        }
    }
    Ok(system)
}

/// Get a new `UserAuth::Trusted` with a dummy identity and no key material, representing
/// the primary user of the local libqaul instance.
fn auth_for_local_user() -> UserAuth {
    let id: Identity = [00; 12].into();
    let k = String::from("local user key");
    UserAuth::Trusted(id, k)
}

/// Get a new `UserAuth::Trusted` with a second dummy identity, representing a user
/// somewhere else on the network.
fn auth_for_remote_user() -> UserAuth {
    let id: Identity = [01; 12].into();
    let k = String::from("remote user key");
    UserAuth::Trusted(id, k)
}

/// Create a Qaul instance with a local user and a remote user, with the same IDs given by
/// `auth_for_local_user` and `auth_for_remote_user`.
fn system_with_users() -> Qaul {
    let mut qaul = Qaul::start();
    qaul.user_inject(auth_for_local_user())
        .expect("Could not create test local user.");
    qaul.user_inject(auth_for_remote_user())
        .expect("Could not create test remote user.");
    qaul
}

#[test]
fn update_user_updates_applied_in_order() {
    use QaulApiEvent::*;
    let auth = auth_for_local_user();

    let update1 = UserUpdate::RealName(Some("Danny Default".into()));
    let update2 = UserUpdate::RealName(Some("Dougie D'Ifferent".into()));

    let qaul = new_fallible_engine(resolve)
        .queue_events(&[
            UpdateUser {
                user: auth_for_local_user(),
                data: update1,
            },
            UpdateUser {
                user: auth_for_local_user(),
                data: update2,
            },
        ])
        .resolve_in_order(system_with_users)
        .expect("Resolution of events failed. Error");

    let user = qaul
        .user_get(auth_for_local_user())
        .expect("Could not get test user.");
    assert_eq!(user.data.real_name, Some(String::from("Dougie D'Ifferent")));
}

#[test]
fn update_user_events_order_independent() {
    use QaulApiEvent::*;
    let auth = auth_for_local_user();

    let prologue: Vec<_> = vec![
        UserUpdate::RealName(Some("Danny Default".into())),
        UserUpdate::DisplayName(Some("danny_default".into())),
    ]
    .into_iter()
    .map(|event| UpdateUser {
        data: event,
        user: auth.clone(),
    })
    .collect();

    let events: Vec<_> = vec![
        UserUpdate::RealName(Some("Dougie D'Ifferent".into())),
        UserUpdate::DisplayName(Some("dougie_different".into())),
    ]
    .into_iter()
    .map(|event| UpdateUser {
        data: event,
        user: auth.clone(),
    })
    .collect();

    let qauls = new_fallible_engine(resolve)
        .queue_prologues(&prologue)
        .queue_events(&events)
        .resolve_all_orders(system_with_users)
        .into_iter()
        .map(|result| result.expect("Resolution of events failed. Error"));

    for qaul in qauls {
        let user = qaul
            .user_get(auth.clone())
            .expect("Could not get test user.");
        assert_eq!(user.data.real_name, Some(String::from("Dougie D'Ifferent")));
        assert_eq!(
            user.data.display_name,
            Some(String::from("dougie_different"))
        );
    }
}

#[test]
fn added_users_can_be_searched() {
    use QaulApiEvent::*;
    let remote_user = User {
        id: auth_for_remote_user().identity(),
        data: UserData {
            real_name: Some(String::from("Jake Coolice")),
            display_name: Some(String::from("jakec1234")),
            ..UserData::default()
        },
    };

    let qaul = new_fallible_engine(resolve)
        .queue_events(&[AddContact {
            user: auth_for_local_user(),
            contact: remote_user.clone(),
        }])
        .resolve_in_order(system_with_users)
        .expect("Resolution of events failed. Error");

    let search_result = qaul
        .contacts_find(auth_for_local_user(), "Jake")
        .expect("Search failed.");
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0], remote_user);
}
