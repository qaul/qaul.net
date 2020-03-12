//! Directory helper to create and manage Alexandria instances

use crate::error::Result;
use std::{fs, path::PathBuf};

/// Create the set of paths that a library depends on
pub(crate) fn scaffold(offset: &PathBuf) -> Result<()> {
    fs::create_dir_all(offset)?;
    fs::create_dir(offset.join("records"))?;
    fs::create_dir(offset.join("meta"))?;
    fs::create_dir(offset.join("cache"))?;
    Ok(())
}

#[test]
fn scaffold_lib() {
    use std::path::Path;
    use tempdir::TempDir;

    let root = TempDir::new("alexandria-test").unwrap();
    let mut offset = root.path().to_path_buf();
    offset.push("library");

    scaffold(&offset).unwrap();
    
    assert!(Path::new(dbg!(&offset.join("records"))).exists());
    assert!(Path::new(dbg!(&offset.join("meta"))).exists());
    assert!(Path::new(dbg!(&offset.join("cache"))).exists());
}
