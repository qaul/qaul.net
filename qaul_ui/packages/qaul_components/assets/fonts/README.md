# Fonts

## NotoColorEmoji.ttf

Bundled so emoji render identically on every platform (see `lib/styles/qaul_emoji.dart`).

- Source: https://github.com/googlefonts/noto-emoji, `2D/fonts/NotoColorEmoji.ttf`
  (commit `1ffdd21391`, font version 2.057). CBDT bitmap build.
- License: SIL Open Font License 1.1, see `NotoColorEmoji-LICENSE.txt`.
- Modified: U+0000, U+000D, U+0020, `#`, `*`, and `0`–`9` are removed from the
  cmap. The font is used as a fallback, and wherever a style's primary family is
  missing (for example `Roboto` on iOS/desktop, or on web before it loads) those
  codepoints would otherwise resolve to the emoji font and render as blank or
  oversized glyphs. Keycap emoji (`#️⃣`, `1️⃣`, ...) still render.

To regenerate from a new upstream release:

```sh
uv run --no-project --with fonttools pyftsubset NotoColorEmoji.ttf \
  --unicodes="$(uv run --no-project --with fonttools python -c "
from fontTools.ttLib import TTFont
drop = {0x00, 0x0D, 0x20, 0x23, 0x2A, *range(0x30, 0x3A)}
print(','.join(f'U+{c:04X}' for c in sorted(set(TTFont('NotoColorEmoji.ttf').getBestCmap()) - drop)))
")" \
  --layout-features='*' --name-IDs='*' --name-languages='*' --notdef-outline \
  --output-file=NotoColorEmoji.subset.ttf
```
