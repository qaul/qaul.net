use libqaul::messages::SigTrust;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Trust {
    Trusted,
    Unverified,
    Invalid,
}

impl From<SigTrust> for Trust {
    fn from(t: SigTrust) -> Self {
        match t {
            SigTrust::Trusted => Trust::Trusted,
            SigTrust::Unverified => Trust::Unverified,
            SigTrust::Invalid => Trust::Invalid,
        }
    }
}

impl From<Trust> for SigTrust {
    fn from(t: Trust) -> Self {
        match t {
            Trust::Trusted => SigTrust::Trusted,
            Trust::Unverified => SigTrust::Unverified,
            Trust::Invalid => SigTrust::Invalid,
        }
    }
}
