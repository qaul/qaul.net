// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'package:flutter/painting.dart';

/// Keeps desktop zoom separate from the system's accessibility text scaling.
class QaulTextScaler extends TextScaler {
  const QaulTextScaler({
    required this.systemTextScaler,
    required this.zoom,
  });

  final TextScaler systemTextScaler;
  final double zoom;

  @override
  double scale(double fontSize) => systemTextScaler.scale(fontSize) * zoom;

  @override
  double get textScaleFactor => scale(14) / 14;

  @override
  bool operator ==(Object other) =>
      other is QaulTextScaler &&
      other.systemTextScaler == systemTextScaler &&
      other.zoom == zoom;

  @override
  int get hashCode => Object.hash(systemTextScaler, zoom);
}
