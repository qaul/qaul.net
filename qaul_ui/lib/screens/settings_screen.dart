import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
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
                const SizedBox(height: 20),
                const _LanguageSelectDropDown(),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _LanguageSelectDropDown extends ConsumerWidget {
  const _LanguageSelectDropDown({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return ValueListenableBuilder(
      valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
      builder: (context, box, _) => DropdownButton<Locale?>(
        value: UserPrefsHelper().defaultLocale,
        items: UserPrefsHelper().supportedLocales.map((value) {
          return DropdownMenuItem<Locale?>(
            value: value,
            child: Text(
              value == null ? 'Use system default' : value.toLanguageTag(),
            ),
          );
        }).toList(),
        onChanged: (val) => UserPrefsHelper().defaultLocale = val,
      ),
    );
  }
}
