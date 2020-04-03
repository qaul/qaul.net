use serde::{Deserialize, Serialize};

/// An in-database path segment
///
/// Each path in the database needs to be unique in it's scope (user
/// or global).  The last identifier in the "seq" chain should be
/// considered the record-id.  This does however not mean that the
/// path becomes blocked for future writes to add further subseqs.
///
/// ## String representation
///
/// An alexandria path can be represented as a string with `:` and `/`
/// acting as delimiters.  Following are some valid examples that can
/// be parsed from strings (or turned into strings).
///
/// - `msg:/private/bob`: msg -> private -> bob
/// - `msg:/private/bob/imgs`: msg -> private -> bob -> imgs
/// - `sessions:/all` -> sessions -> all
///
/// The following code can be used to create Path objects from
/// strings:
///
/// ```rust
/// # use alexandria::Path;
/// let _: Path = "test:/data".into();
/// ```
///
/// ## Util macro
///
/// If you want to create paths from identifiers, not just strings, or
/// if you want to avoid stringly typed paths, you can also use the
/// `mkPath!` macro in the same module.
///
/// ```rust
/// # use alexandria::path::mkPath;
/// let _: Path = mkPath!("imgs", "bob", "cool");
/// ```
///
///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Path {
    root: String,
    zones: Vec<String>,
}

impl From<&Path> for String {
    fn from(p: &Path) -> String {
        format!("{}:{}", p.root, p.zones.join("/"))
    }
}

impl From<String> for Path {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'path> From<&'path str> for Path {
    fn from(s: &'path str) -> Self {
        Self {
            root: s.into(),
            zones: vec![],
        }
    }
}
