import 'package:flutter/material.dart';

enum QaulColorMode { light, dark }

class QaulColorSheet {
  QaulColorMode mode = QaulColorMode.light;

  Color get background =>
      mode == QaulColorMode.dark ? const Color(0xFF000000) : Colors.white;

  Color get surfaceContainer => mode == QaulColorMode.dark
      ? const Color(0xFF333333)
      : const Color(0xFFE5E5E5);
}
