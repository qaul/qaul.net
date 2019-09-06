/// A simple storable interface
///
/// Interacting with a storable object requires knowing
/// it's disk-offset (which is enforced by it's scope)
/// as well as it's identity name (also known by the scope).
///
/// While normally some entities don't need to know their
/// name to be useful, for disk I/O this becomes fundamental.
/// Some types might want to implement these functions only
/// by passing calls off to children, augmenting them with
/// the proper metadata.
pub(crate) trait Storable {
    /// Read the selected type from disk
    fn read(offset: &str, name: &str) -> Self;

    /// Write the selected type to disk
    fn write(&self, offset: &str, name: &str);
}
