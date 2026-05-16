import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:widgetbook/widgetbook.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'main.directories.g.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

/// Penpot / designer breakpoints only (no preset device catalog).
const List<ViewportData> _kDesignerViewports = [
  ViewportData(
    name: 'iPhone SE',
    width: 375,
    height: 667,
    pixelRatio: 2,
    platform: TargetPlatform.iOS,
  ),
  ViewportData(
    name: 'iPhone 16',
    width: 393,
    height: 852,
    pixelRatio: 3,
    platform: TargetPlatform.iOS,
  ),
  ViewportData(
    name: 'MIUI',
    width: 393,
    height: 851,
    pixelRatio: 3,
    platform: TargetPlatform.android,
  ),
  ViewportData(
    name: 'Google Pixel 9 Pro',
    width: 427,
    height: 952,
    pixelRatio: 3,
    platform: TargetPlatform.android,
  ),
  ViewportData(
    name: 'Laptop screen black',
    width: 1366,
    height: 768,
    pixelRatio: 2,
    platform: TargetPlatform.linux,
  ),
];

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  // ignore: invalid_use_of_visible_for_testing_member
  SharedPreferences.setMockInitialValues({});
  runApp(const WidgetbookApp());
}

@widgetbook.App()
class WidgetbookApp extends StatelessWidget {
  const WidgetbookApp({super.key});

  @override
  Widget build(BuildContext context) {
    return Widgetbook.material(
      directories: directories,
      addons: [
        ViewportAddon([
          Viewports.none,
          ..._kDesignerViewports,
        ]),
        LocalizationAddon(
          locales: QaulComponentsLocalizations.supportedLocales,
          localizationsDelegates: const [
            QaulComponentsLocalizations.delegate,
            GlobalMaterialLocalizations.delegate,
            GlobalWidgetsLocalizations.delegate,
            GlobalCupertinoLocalizations.delegate,
          ],
          initialLocale: const Locale('en'),
        ),
        MaterialThemeAddon(
          themes: [
            WidgetbookTheme(name: 'Light', data: QaulAppTheme.light),
            WidgetbookTheme(name: 'Dark', data: QaulAppTheme.dark),
          ],
        ),
      ],
    );
  }
}
