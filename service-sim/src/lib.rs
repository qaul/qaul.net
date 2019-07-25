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

#[test]
fn update_user_applies_to_find_and_get() {
    use QaulApiEvent::*;
    let auth = test_auth();

    let update1 = UserData::new().with_real_name("Danny Default");
    let update2 = update1.clone().with_display_name("@dannydefault");

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
    let by_user_name_query = qaul.contacts_find(auth.clone(), "@dannydefault").unwrap();
    let by_real_name_query = qaul.contacts_find(auth.clone(), "Danny Default").unwrap();
    let by_wrong_name_query = qaul.contacts_find(auth.clone(), "Dougie").unwrap();
    let all_contacts = qaul.contacts_get_all(auth.clone()).unwrap();

    assert_eq!(by_user_name_query.len(), 1);
    assert_eq!(by_real_name_query.len(), 1);
    assert_eq!(by_wrong_name_query.len(), 0);
    assert_eq!(all_contacts.len(), 1);
}
