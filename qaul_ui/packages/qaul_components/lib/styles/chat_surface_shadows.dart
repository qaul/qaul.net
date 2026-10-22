import 'package:flutter/material.dart';

/// The side of a chat surface on which its shadow is cast.
enum ChatSurfaceShadowDirection { above, below }

/// Returns the shared shadow treatment for chat headers and footers.
///
/// Headers cast their shadow [ChatSurfaceShadowDirection.below], while footers
/// cast it [ChatSurfaceShadowDirection.above]. Keeping the values here makes
/// the two surfaces symmetrical as the design evolves.
List<BoxShadow> chatSurfaceShadows(
  Brightness brightness, {
  required ChatSurfaceShadowDirection direction,
}) {
  final isDark = brightness == Brightness.dark;
  return [
    BoxShadow(
      offset: Offset.zero,
      blurRadius: 5,
      color: isDark ? const Color(0x66000000) : const Color(0x33000000),
    ),
    BoxShadow(
      offset: Offset(
        0,
        direction == ChatSurfaceShadowDirection.below ? 10 : -10,
      ),
      blurRadius: 7,
      color: isDark ? const Color(0x99000000) : const Color(0x66000000),
    ),
  ];
}
