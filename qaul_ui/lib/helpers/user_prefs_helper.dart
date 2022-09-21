import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';

class UserPrefsHelper {
  UserPrefsHelper() : _prefsBox = Hive.box(hiveBoxName);
  final Box<dynamic>? _prefsBox;

  static String get hiveBoxName => 'UserPreferencesBox';

  String get _defaultLocaleKey => 'cached_default_locale';

  String get _defaultThemeKey => 'cached_default_theme';

  String get _publicNTFYKey => 'cached_public_notification_enabled';

  String get _chatNTFYKey => 'cached_chat_notification_enabled';

  String get _verifyNTFKey => 'cached_verified_users_only';

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

  bool get publicTabNotificationsEnabled => _prefsBox?.get(_publicNTFYKey) ?? true;

  set publicTabNotificationsEnabled(bool val) => _prefsBox?.put(_publicNTFYKey, val);

  bool get chatNotificationsEnabled => _prefsBox?.get(_chatNTFYKey) ?? true;

  set chatNotificationsEnabled(bool val) => _prefsBox?.put(_chatNTFYKey, val);

  bool get notifyOnlyForVerifiedUsers => _prefsBox?.get(_verifyNTFKey) ?? false;

  set notifyOnlyForVerifiedUsers(bool v) => _prefsBox?.put(_verifyNTFKey, v);
}
