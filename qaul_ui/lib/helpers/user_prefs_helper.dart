import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';

class UserPrefsHelper {
  UserPrefsHelper() : _prefsBox = Hive.box(hiveBoxName);
  final Box<dynamic>? _prefsBox;

  static String get hiveBoxName => 'UserPreferencesBox';

  String get _defaultLocaleKey => 'cached_default_locale';

  String get _defaultThemeKey => 'cached_default_theme';

  List<Locale?> get supportedLocales => [
        null,
        const Locale.fromSubtags(languageCode: 'en'),
        const Locale.fromSubtags(languageCode: 'ar'),
        const Locale.fromSubtags(languageCode: 'pt'),
      ];

  Locale? get defaultLocale {
    String? completeCode = _prefsBox?.get(_defaultLocaleKey);
    if (completeCode == null) return null;
    final cs = completeCode.split('_');

    if (cs.last == 'null') return Locale.fromSubtags(languageCode: cs.first);
    return Locale.fromSubtags(languageCode: cs.first, countryCode: cs.last);
  }

  set defaultLocale(Locale? l) {
    String? code;
    if (l != null) code = '${l.languageCode}_${l.countryCode}';
    _prefsBox?.put(_defaultLocaleKey, code);
  }

  ThemeMode get defaultTheme {
    int? theme = _prefsBox?.get(_defaultThemeKey);
    if (theme == null) return ThemeMode.system;

    return ThemeMode.values[theme.clamp(0, ThemeMode.values.length - 1)];
  }

  set defaultTheme(ThemeMode theme) {
    _prefsBox?.put(_defaultThemeKey, ThemeMode.values.indexOf(theme));
  }
}
