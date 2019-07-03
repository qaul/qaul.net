//! Platform specific utilities

/// Platform types that `libqaul` supports
pub enum Platform {
    #[cfg(any(
        features = "pc_linux",
        features = "pc_macos",
        features = "pc_windows",
        features = "pc_generic"
    ))]
    Desktop(DesktopType),
    #[cfg(any(
        features = "mobile_android",
        features = "mobile_ios",
        features = "mobile_generic"
    ))]
    Mobile(MobileType),
    None,
}


/// Exact type of desktop platform
#[cfg(any(
    features = "pc_linux",
    features = "pc_macos",
    features = "pc_windows",
    features = "pc_generic"
))]
pub enum DesktopType {
    #[cfg(features = "pc_linux")]
    Linux,
    #[cfg(features = "pc_macos")]
    MacOS,
    #[cfg(features = "pc_windows")]
    Windows,
    #[cfg(features = "pc_generic")]
    Generic,
}

/// Exact type of mobile platform
#[cfg(any(
    features = "mobile_android",
    features = "mobile_ios",
    features = "mobile_generic"
))]
pub enum MobileType {
    #[cfg(features = "mobile_android")]
    Android,
    #[cfg(features = "mobile_ios")]
    iOS,
    #[cfg(features = "mobile_generic")]
    Generic,
}
