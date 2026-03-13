import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:widgetbook/widgetbook.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'main.directories.g.dart';

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
          const ViewportData(
            name: kBreakpointIphone16,
            width: 393,
            height: 852,
            pixelRatio: 2.0,
            platform: TargetPlatform.iOS,
          ),
          Viewports.none,
          ...kDesignerBreakpoints
              .where((v) => v.name != kBreakpointIphone16)
              .map(
                (v) => ViewportData(
                  name: v.name,
                  width: v.width,
                  height: v.height,
                  pixelRatio: 2.0,
                  platform: v.name == kBreakpointIphone16Pro
                      ? TargetPlatform.iOS
                      : TargetPlatform.linux,
                ),
              ),
          ...IosViewports.phones,
          ...IosViewports.tablets,
          AndroidViewports.samsungGalaxyS20,
          AndroidViewports.samsungGalaxyNote20,
          LinuxViewports.desktop,
        ]),
        LocalizationAddon(
          locales: const [Locale('en')],
          localizationsDelegates: GlobalMaterialLocalizations.delegates,
          initialLocale: const Locale('en'),
        ),
        MaterialThemeAddon(
          themes: [
            WidgetbookTheme(
              name: 'Light',
              data: ThemeData(
                useMaterial3: true,
                brightness: Brightness.light,
                colorScheme: ColorScheme.fromSeed(
                  seedColor: Colors.lightBlue,
                ),
              ),
            ),
            WidgetbookTheme(
              name: 'Dark',
              data: ThemeData(
                useMaterial3: true,
                brightness: Brightness.dark,
                colorScheme: ColorScheme.fromSeed(
                  seedColor: Colors.lightBlue,
                  brightness: Brightness.dark,
                ),
              ),
            ),
          ],
        ),
      ],
    );
  }
}
