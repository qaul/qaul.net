import 'package:flutter/material.dart';

import '../../styles/qaul_color_sheet.dart';

/// Shared style for centered chat meta text (date dividers, join/rename
/// notices). Kept in one place so the meta renderers can't drift apart.
TextStyle chatMetaTextStyle(BuildContext context) => TextStyle(
  fontSize: 12,
  height: 1.2,
  color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.55),
);

TextStyle chatDateDividerTextStyle(BuildContext context) => TextStyle(
  fontFamily: 'Roboto',
  fontSize: 12,
  fontWeight: FontWeight.w300,
  height: 1.2,
  letterSpacing: 0.2,
  color: QaulColorSheet(Theme.of(context).brightness).chatDateDividerText,
);
