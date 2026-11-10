import 'package:flutter/material.dart';

const _kLightChatSurfaceShadow = Color(0x33000000);
const _kDarkChatSurfaceShadow = Color(0x66000000);
const _kLightChatSurfaceDivider = Color(0xFFD1D1D6);
const _kDarkChatSurfaceDivider = Color(0xFF9E9E9E);

/// The side of a chat surface on which its shadow is cast.
enum ChatSurfaceShadowDirection { above, below }

/// The shared divider color for chat headers and footers.
Color chatSurfaceDividerColor(Brightness brightness) =>
    brightness == Brightness.dark
    ? _kDarkChatSurfaceDivider
    : _kLightChatSurfaceDivider;

/// The shared divider treatment for chat headers and footers.
BorderSide chatSurfaceStroke(Brightness brightness) =>
    BorderSide(color: chatSurfaceDividerColor(brightness), width: 0.5);

/// Returns the shared shadow treatment for chat headers and footers.
///
/// Headers cast their shadow [ChatSurfaceShadowDirection.below], while footers
/// cast it [ChatSurfaceShadowDirection.above]. Keeping the values here makes
/// the two surfaces symmetrical as the design evolves.
List<BoxShadow> chatSurfaceShadows(
  Brightness brightness, {
  required ChatSurfaceShadowDirection direction,
}) {
  if (brightness == Brightness.light) {
    return const [
      BoxShadow(
        offset: Offset.zero,
        blurRadius: 5,
        color: _kLightChatSurfaceShadow,
      ),
    ];
  }

  return [
    BoxShadow(
      offset: Offset(
        0,
        direction == ChatSurfaceShadowDirection.below ? 10 : -10,
      ),
      blurRadius: 7,
      color: _kDarkChatSurfaceShadow,
    ),
  ];
}
