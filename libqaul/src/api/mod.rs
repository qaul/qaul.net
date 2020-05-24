//! A set of function specific API scopes in libqaul
//!
//! Because libqaul is a rather wide library (meaning that it manages
//! many things), the set of functions available are broken into
//! separate scopes, to make reasoning about association easier.
//!
//! All scopes can be constructed via the main
//! [`Qaul`][../struct.Qaul.html] type access functions.
//!
//! ```
//! # let qaul = libqaul::Qaul::dummy();
//! let users = qaul.users();
//! let messages = qaul.messages();
//! let contacts = qaul.contacts();
//! let services = qaul.services();
//! ```
//!
//! Each scope manages only one aspect of the libqaul instance, all of
//! which are usually required to run a network service.

pub use contacts::Contacts;
pub(crate) mod contacts;

pub use messages::Messages;
pub(crate) mod messages;

pub use services::Services;
pub(crate) mod services;

pub use users::Users;
pub(crate) mod users;
