//! RATMAN networking frames
//!
//! A `frame` is the smallest individual packet that can be sent over a connection.
//! It is comparable to a UDP or TCP packet, holding a limited amount of data,
//! depending on a lot of factors.
//!
//! For now, the `frames` module defines some basic data types
//! that are important for RATMAN to function.


use qaul_common::UserID;

pub struct Header {
    to: UserID,
    from: UserID,
}