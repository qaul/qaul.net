/// Bundled Noto Color Emoji family, so emoji look the same on every platform
/// instead of falling back to Apple, Segoe, or whatever the OS ships.
///
/// Fonts declared by a package must be referenced with the `packages/<name>/`
/// prefix when used as a fallback from outside that package.
const String kQaulEmojiFontFamily = 'packages/qaul_components/NotoColorEmoji';

/// Use as [TextStyle.fontFamilyFallback] (or [ThemeData.fontFamilyFallback])
/// wherever text may contain emoji.
const List<String> kQaulEmojiFontFamilyFallback = [kQaulEmojiFontFamily];
