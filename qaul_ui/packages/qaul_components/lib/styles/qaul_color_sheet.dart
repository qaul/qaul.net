import 'package:flutter/material.dart';

import 'chat_surface_shadows.dart';

class QaulColorSheet {
  const QaulColorSheet(this.brightness);

  final Brightness brightness;

  bool get _isDark => brightness == Brightness.dark;

  Color get background => _isDark ? const Color(0xFF000000) : Colors.white;

  Color get surfaceContainer =>
      _isDark ? const Color(0xFF333333) : const Color(0xFFE5E5E5);

  Color get chatBubbleSelectionOutline => _isDark ? Colors.white : Colors.black;

  Color get chatSurfaceDivider => chatSurfaceDividerColor(brightness);

  Color get chatFooterDivider => chatSurfaceDivider;

  Color get chatHeaderDivider => chatSurfaceDivider;

  Color get chatDateDividerText => _isDark ? Colors.white : Colors.black;
}
