//! A set of API helpers

mod subs;
pub use subs::{SubId, Subscription};

mod diff;
pub use diff::{ItemDiff, ItemDiffExt, MapDiff, MapDiffExt, SetDiff, SetDiffExt};

mod query;
pub use query::QueryResult;

/// A searchable metadata tag for messages and data files
///
/// Data that is stored and managed by libqaul can be tagged to make
/// searching easier and faster, because the backing storage is
/// encrypted and might not be immediately accessible.
///
/// It also allows a service to tag data with specific parameters that
/// will be relevant to it's implementation, without having to teach
/// libqaul about what they mean (for example `room-id` for
/// `net.qaul.chat`).
pub type Tag = alexandria::utils::Tag;

pub use alexandria::utils::TagSet;

/// Generate tags for types dynamically to associate records
///
/// When dealing with subscriptions or iterators over queries, the
/// types that are returned from libqaul all stem from the same
/// underlying representation of types in the storage library.
/// Because there's no good way to re-assosiate which types can be
/// turned into what, and because it will cause unneccessary cycles in
/// subscribers (that might be polled via a remote interface), there
/// needs to be a way to filter out subscriptions based on the types
/// that they expect.
///
/// This is (as most things in libqaul) done via search tags.  Any
/// type that wants to be returned via a subscription or query
/// iterator needs to implement this trait so that alexandria can
/// create a forced bound on this type tag.
pub trait Tagged {
    fn tag() -> Tag;
}
