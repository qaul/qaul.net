pub mod danny {
    use crate::QaulApiEvent;
    use libqaul::{Identity, UserAuth, UserUpdate, User, UserData};
    /// Return the privelaged UserAuth for "Danny Default"
    pub fn auth() -> UserAuth {
        let id: Identity = [00; 12].into();
        let k = String::from("danny");
        UserAuth::Trusted(id, k)
    }

    /// Return the standard user data for "Danny Default"
    pub fn user() -> User {
        User {
            id: auth().identity(),
            data: UserData {
                real_name: Some(String::from("Danny Default")),
                display_name: Some(String::from("dannydefault")),
                ..UserData::default()
            },
        }
    }

    /// Return the events use to create "Danny Default"
    pub fn creation_events() -> Vec<QaulApiEvent> {
        use QaulApiEvent::*;
        vec![
            InjectUser { user: auth() },
            UpdateUser { user: auth(), data: UserUpdate::RealName(user().data.real_name) },
            UpdateUser { user: auth(), data: UserUpdate::DisplayName(user().data.display_name) }
        ]
    }
}

pub mod jake {
    use crate::QaulApiEvent;
    use libqaul::{Identity, UserAuth, UserUpdate, User, UserData};
    /// Return the privelaged UserAuth for "Jake Coolice"
    pub fn auth() -> UserAuth {
        let id: Identity = [01; 12].into();
        let k = String::from("jake");
        UserAuth::Trusted(id, k)
    }

    /// Return the standard user data for "Jake Coolice"
    pub fn user() -> User {
        User {
            id: auth().identity(),
            data: UserData {
                real_name: Some(String::from("Jake Coolice")),
                display_name: Some(String::from("jakec1234")),
                ..UserData::default()
            },
        }
    }
    
    /// Return the events used to create "Jake Coolice"
    pub fn creation_events() -> Vec<QaulApiEvent> {
        use QaulApiEvent::*;
        vec![
            InjectUser { user: auth() },
            UpdateUser { user: auth(), data: UserUpdate::RealName(user().data.real_name) },
            UpdateUser { user: auth(), data: UserUpdate::DisplayName(user().data.display_name) }
        ]
    }
}
