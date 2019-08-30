#![cfg(test)]
use libqaul::{Identity, Qaul, QaulResult, UserAuth, UserData, UserUpdate};
use visn::{new_fallible_engine, KnowledgeEngine};

#[derive(Clone)]
enum QaulApiEvent {
    UpdateUser { user: UserAuth, data: UserUpdate },
    GetUser { user: UserAuth },
    DeleteUser { user: UserAuth },
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

    let update1 = UserUpdate::RealName(Some("Danny Default".into()));
    let update2 = UserUpdate::RealName(Some("Dougie D'Ifferent".into()));

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

#[test]
fn update_user_events_order_independent() {
    use QaulApiEvent::*;
    let auth = test_auth();

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
        .resolve_all_orders(system_with_auth)
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
