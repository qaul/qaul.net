import 'dart:developer';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;
import 'package:qaul_ui/l10n/app_localizations.dart';

part 'screenshot_comparator.dart';

part 'test_responsive_widgets.dart';

Widget materialAppWithLocalizations(Widget wut) {
  return MaterialApp(
    localizationsDelegates: AppLocalizations.localizationsDelegates,
    supportedLocales: AppLocalizations.supportedLocales,
    home: Material(child: Builder(builder: (context) => wut)),
  );
}
