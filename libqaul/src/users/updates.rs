use crate::users::UserData;

/// All the ways a UserData can change, as individual events.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UserUpdate {
    /// Set or blank the User's display name.
    DisplayName(Option<String>),
    /// Set or blank the User's real name.
    RealName(Option<String>),
    /// Add or update a biography line with the given key to the given value.
    SetBioLine(String, String),
    /// Remove a biography line with the given key, or do nothing if it does not exist.
    RemoveBioLine(String),
    /// Add a service with the given name.
    AddService(String),
    /// Remove the service with the given name, or do nothing if it does not exist.
    RemoveService(String),
    /// Set or blank the User's avatar.
    AvatarData(Option<Vec<u8>>),
}

impl UserUpdate {
    /// Change the given UserData based on the instruction given by this UserUpdate.
    pub fn apply_to(self, data: &mut UserData) {
        use UserUpdate::*;
        match self {
            DisplayName(v) => data.display_name = v,
            RealName(v) => data.real_name = v,
            SetBioLine(k, v) => {
                data.bio.insert(k, v);
            }
            RemoveBioLine(k) => {
                data.services.remove(&k);
            }
            AddService(k) => {
                data.services.insert(k);
            }
            RemoveService(k) => {
                data.services.remove(&k);
            }
            AvatarData(v) => data.avatar = v,
        }
    }
}
