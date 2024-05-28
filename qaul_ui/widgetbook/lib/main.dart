import 'package:flutter/material.dart';

import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:qaul_ui/qaul_app.dart';
import 'package:widgetbook/widgetbook.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

// This file does not exist yet,
// it will be generated in the next step
import 'main.directories.g.dart';

void main() {
  runApp(const WidgetbookApp());
}

@widgetbook.App()
class WidgetbookApp extends StatelessWidget {
  const WidgetbookApp({super.key});

  @override
  Widget build(BuildContext context) {
    return Widgetbook.material(
      // The [directories] variable does not exist yet,
      // it will be generated in the next step
      directories: directories,
      addons: [
        DeviceFrameAddon(devices: [
          Devices.android.smallPhone,
          Devices.android.mediumPhone,
          Devices.android.bigPhone,
          Devices.linux.laptop,
          Devices.macOS.macBookPro,
        ]),
        LocalizationAddon(
          locales: AppLocalizations.supportedLocales,
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          initialLocale: AppLocalizations.supportedLocales.last,
        ),
        MaterialThemeAddon(
          themes: [
            WidgetbookTheme(
              name: 'Light',
              data: QaulApp.lightTheme,
            ),
            WidgetbookTheme(
              name: 'Dark',
              data: QaulApp.darkTheme,
            ),
          ],
          // initialTheme: WidgetbookTheme(
          //   name: 'Light',
          //   data: yourMaterialLightTheme,
          // ),
        ),
      ],
    );
  }
}
