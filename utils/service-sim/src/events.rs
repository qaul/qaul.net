use libqaul::{Qaul, QaulResult, UserAuth, User, UserUpdate};

/// An enum representing all the various operations supported by the Qaul API.
/// These are only the currently implemented operations.
///
/// ## Adding New Events
///
/// When adding an event, add it here and in the `resolve` function. If you don't do so,
/// the module will fail to compile, as that match is meant to be exhaustive. Please also
/// add a doc comment explaining what that event does.
///
/// ## Coventions
/// As used here, the `user` is the user *performing* an action. For instance, in `AddContact`,
/// the `user` is the user to whose contact book the `contact` is added.
#[derive(Clone)]
pub enum QaulApiEvent {
    /// Inject the given UserAuth, creating a new user from thin air.
    InjectUser { user: UserAuth },
    /// Update the given user's information with the given UserUpdate.
    UpdateUser { user: UserAuth, data: UserUpdate },
    /// Delete the given user.
    DeleteUser { user: UserAuth },
    /// Add the given contact to the given user's contact book.
    AddContact { user: UserAuth, contact: User },
}

/// Resolve `QaulApiEvent`s by modifying the given `Qaul` instance. For use with `visn`
/// test runner functions. Might fail, so you need the fallible variant of `visn`'s
/// `KnowledgeEngine`.
pub fn resolve(event: QaulApiEvent, system: Qaul) -> QaulResult<Qaul> {
    use QaulApiEvent::*;
    match event {
        InjectUser { user } => {
            system.user_inject(user)?;
        }
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
