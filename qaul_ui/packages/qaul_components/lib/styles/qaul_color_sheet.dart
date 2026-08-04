import 'package:flutter/material.dart';

class QaulColorSheet {
  const QaulColorSheet(this.brightness);

  static const _lightChatDivider = Color(0xFFD1D1D6);
  static const _darkHeaderDivider = Color(0xFF9E9E9E);

  final Brightness brightness;

  bool get _isDark => brightness == Brightness.dark;

  Color get background => _isDark ? const Color(0xFF000000) : Colors.white;

  Color get surfaceContainer =>
      _isDark ? const Color(0xFF333333) : const Color(0xFFE5E5E5);

  Color get chatBubbleSelectionOutline => _isDark ? Colors.white : Colors.black;

  Color get chatFooterDivider => _isDark ? Colors.white : _lightChatDivider;

  Color get chatHeaderDivider =>
      _isDark ? _darkHeaderDivider : _lightChatDivider;
}
