import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

class UserPrefsHelper {
  UserPrefsHelper._internal(this._prefs);
  
  static UserPrefsHelper? _instance;
  final SharedPreferencesWithCache _prefs;
  final ValueNotifier<int> _changeNotifier = ValueNotifier(0);
  
  static Future<void> initialize() async {
    final prefs = await SharedPreferencesWithCache.create(
      cacheOptions: const SharedPreferencesWithCacheOptions(),
    );
    _instance = UserPrefsHelper._internal(prefs);
  }
  
  factory UserPrefsHelper() {
    assert(_instance != null, 'Call UserPrefsHelper.initialize() first!');
    return _instance!;
  }
  
  ValueListenable<int> get listenable => _changeNotifier;
  
  void _notifyChange() {
    _changeNotifier.value++;
  }

  String get _defaultLocaleKey => 'cached_default_locale';

  String get _defaultThemeKey => 'cached_default_theme';

  String get _publicNTFYKey => 'cached_public_notification_enabled';

  String get _chatNTFYKey => 'cached_chat_notification_enabled';

  String get _verifyNTFKey => 'cached_verified_users_only';

  Locale? get defaultLocale {
    String? completeCode = _prefs.getString(_defaultLocaleKey);
    if (completeCode == null) return null;
    final cs = completeCode.split('_');

    if (cs.last == 'null') return Locale.fromSubtags(languageCode: cs.first);
    return Locale.fromSubtags(languageCode: cs.first, countryCode: cs.last);
  }

  set defaultLocale(Locale? l) {
    if (l == null) {
      _prefs.remove(_defaultLocaleKey);
    } else {
      String code = '${l.languageCode}_${l.countryCode}';
      _prefs.setString(_defaultLocaleKey, code);
    }
    _notifyChange();
  }

  ThemeMode get defaultTheme {
    int? theme = _prefs.getInt(_defaultThemeKey);
    if (theme == null) return ThemeMode.system;

    return ThemeMode.values[theme.clamp(0, ThemeMode.values.length - 1)];
  }

  set defaultTheme(ThemeMode theme) {
    _prefs.setInt(_defaultThemeKey, ThemeMode.values.indexOf(theme));
    _notifyChange();
  }

  bool get publicTabNotificationsEnabled => _prefs.getBool(_publicNTFYKey) ?? true;

  set publicTabNotificationsEnabled(bool val) {
    _prefs.setBool(_publicNTFYKey, val);
    _notifyChange();
  }

  bool get chatNotificationsEnabled => _prefs.getBool(_chatNTFYKey) ?? true;

  set chatNotificationsEnabled(bool val) {
    _prefs.setBool(_chatNTFYKey, val);
    _notifyChange();
  }

  bool get notifyOnlyForVerifiedUsers => _prefs.getBool(_verifyNTFKey) ?? false;

  set notifyOnlyForVerifiedUsers(bool v) {
    _prefs.setBool(_verifyNTFKey, v);
    _notifyChange();
  }
}
