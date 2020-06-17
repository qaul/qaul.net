//! # Qaul service-sim
//! Simulation and testing of libqaul API usage using `visn` and locally defined data-
//! types. This is the crate in which integration testing of the `libqaul` Qaul API is
//! confined.
//!
//! ## Writing a new test
//!
//! Tests follows the "given/when/then" structure; *given* certain pre-conditions, *when*
//! certain actions are performed, *then* the state of the system will be just so.
//!
//! For example, *given* a newly created Qaul instance, *when* a user's name is updated,
//! *then* the user's data will have the new name.
//!
//! `visn` is used to implement different orderings of events. See the documentation of
//! that crate and the tests that already exist for additional details. In essence,
//! `visn` is given the `resolve` function from the `events` module and a list of events
//! and processes them to create either a single Qaul instance or a `Vec` of all possible
//! Qaul instances for each possible ordering of those events.
//!
//! ## Adding new events
//!
//! If you have implemented a new piece of the API, you will need to add a corresponding
//! event in the `events` module. See the documentation there for more info.
//!

#![doc(html_favicon_url = "https://qaul.net/favicon.ico")]
#![doc(html_logo_url = "https://qaul.net/img/qaul_icon-128.png")]

use libqaul::{Identity, Qaul, QaulResult, User, UserAuth, UserData, UserUpdate};
use visn::{new_fallible_engine, KnowledgeEngine};

/// The events that define mutations on the state of `libqaul`.
pub mod events;
pub use events::{QaulApiEvent, resolve};

/// Sample users for use in tests, each with their own unique identity and data.
///
/// Each user has a name, which is its own module's identifier. Each name should begin
/// with a unique letter (and if we end up with more than 24 users, this system should
/// probably be refined into a less hard-coded one).
///
/// Each user should provide `::auth()`, giving a privelaged `UserAuth`; `::user()`, giving
/// a full `User` with some data, and `::creation_events()`, the events needed to create
/// the `User` (to be used with .queue_prologues()).
pub mod users;

#[test]
fn update_user_updates_applied_in_order() {
    use QaulApiEvent::*;
    let qaul = new_fallible_engine(resolve)
        .queue_prologue(InjectUser { user: users::danny::auth() })
        .queue_events(&[
            UpdateUser {
                user: users::danny::auth(),
                data: UserUpdate::DisplayName(Some("danny_d".into())),
            },
            UpdateUser {
                user: users::danny::auth(),
                data: UserUpdate::RealName(Some("ddefault".into())),
            },
        ])
        .resolve_in_order(Qaul::start)
        .expect("Resolution of events failed. Error");

    let user = qaul
        .user_get(users::danny::auth())
        .expect("Could not get test user.");
    assert_eq!(user.data.display_name, Some(String::from("danny_d")));
}

#[test]
fn users_can_be_retrieved_and_searched() {
    use QaulApiEvent::*;
    let qaul = new_fallible_engine(resolve)
        .queue_prologues(&users::danny::creation_events())
        .queue_events(&[AddContact {
            user: users::danny::auth(),
            contact: users::jake::user(),
        }])
        .resolve_in_order(Qaul::start)
        .expect("Resolution of events failed. Error");

    let search_result = qaul
        .contacts_find(users::danny::auth(), "Jake")
        .expect("Search failed.");
    let fetch_result = qaul
        .contacts_get_all(users::danny::auth())
        .expect("Fetch failed.");
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0], users::jake::user());
    assert_eq!(fetch_result.len(), 1);
    assert_eq!(fetch_result[0], users::jake::user());
}

#[test]
fn contacts_retrieval_exclusive_across_identities() {
    use QaulApiEvent::*;
    let qaul = new_fallible_engine(resolve)
        .queue_prologues(&users::danny::creation_events())
        .queue_prologues(&users::jake::creation_events())
        .queue_events(&[
            AddContact {
                user: users::danny::auth(),
                contact: users::jake::user(),
            },
            AddContact {
                user: users::jake::auth(),
                contact: users::danny::user(),
            },
        ])
        .resolve_in_order(Qaul::start)
        .expect("Resolution of events failed. Error");

    let fetch_as_danny = qaul
        .contacts_get_all(users::danny::auth())
        .expect("Search failed.");
    let fetch_as_jake = qaul
        .contacts_get_all(users::jake::auth())
        .expect("Search failed.");
    assert_eq!(fetch_as_danny.len(), 1);
    assert_eq!(fetch_as_jake.len(), 1);
    assert_eq!(fetch_as_danny[0], users::jake::user());
    assert_eq!(fetch_as_jake[0], users::danny::user());
}

#[test]
fn update_user_events_order_independent() {
    use QaulApiEvent::*;
    let qauls = new_fallible_engine(resolve)
        .queue_prologues(&users::danny::creation_events())
        .queue_events(&[
            UpdateUser {
                user: users::danny::auth(),
                data: UserUpdate::RealName(Some("Dougie D'Ifferent".into())),
            },
            UpdateUser {
                user: users::danny::auth(),
                data: UserUpdate::DisplayName(Some("dougie_different".into())),
            }
        ])
        .resolve_all_orders(Qaul::start)
        .into_iter()
        .map(|result| result.expect("Resolution of events failed. Error"));

    for qaul in qauls {
        let user = qaul
            .user_get(users::danny::auth())
            .expect("Could not get test user.");
        assert_eq!(user.data.real_name, Some(String::from("Dougie D'Ifferent")));
        assert_eq!(
            user.data.display_name,
            Some(String::from("dougie_different"))
        );
    }
}
