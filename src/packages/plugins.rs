
use serde::{Serialize, de::DeserializeOwned};

/// A simple trait that allows querying the type of 
/// data that is stored in a package via a service
pub trait ExternData: Serialize + DeserializeOwned {
    fn get_type(&self) -> String;
    fn get_size(&self) -> u64;
}


