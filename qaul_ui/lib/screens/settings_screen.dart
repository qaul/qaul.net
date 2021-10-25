import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20.0),
        child: Column(
          children: [
            Row(
              children: [
                const Text('Dark mode:'),
                ValueListenableBuilder<AdaptiveThemeMode>(
                  valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
                  builder: (_, mode, child) {
                    var isDark = mode == AdaptiveThemeMode.dark;
                    return Switch(
                      value: isDark,
                      onChanged: (_) => isDark
                          ? AdaptiveTheme.of(context).setLight()
                          : AdaptiveTheme.of(context).setDark(),
                    );
                  },
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
