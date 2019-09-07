//! A change applied to a databse

/// A delta object applied to a record
///
/// Is always transferred in combination with an `Address`, which
/// specifies where in a Library to apply a change. Some functions
/// maybe act on `Vec<Delta>`, meaning that their operations can be
/// chained. In this case any dependencies between Deltas need to be
/// resolved before building the `Vec`.
///
/// Generally a `Delta` is a single insert, delete or update.
pub enum Delta<T> {
    /// Insert a new dataset into an address. If the record previously
    /// existed, it will be overridden.
    Insert(T),
    /// Delete a record from the library
    Delete,
    /// An update is a soft-override. New fields will be added,
    /// existing fields may be override, but none will be
    /// deleted. This is a purely additive action.
    ///
    /// To override data in an existing slot, use `Insert(Data)`
    /// instead!
    Update(T),
}
