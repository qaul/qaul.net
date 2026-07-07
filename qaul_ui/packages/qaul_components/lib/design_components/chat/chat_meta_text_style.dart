import 'package:flutter/material.dart';

/// Shared style for centered chat meta text (date dividers, join/rename
/// notices). Kept in one place so the meta renderers can't drift apart.
TextStyle chatMetaTextStyle(BuildContext context) => TextStyle(
  fontSize: 12,
  height: 1.2,
  color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.55),
);
