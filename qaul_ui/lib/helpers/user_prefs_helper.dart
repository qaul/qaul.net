import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class UserPrefsHelper {
  UserPrefsHelper._internal(this._prefs);

  static UserPrefsHelper? _instance;

  static UserPrefsHelper get instance {
    assert(_instance != null, 'Call UserPrefsHelper.initialize() first');
    return _instance!;
  }

  final SharedPreferencesAsync _prefs;

  static const _defaultLocaleKey = 'cached_default_locale';
  static const _themeModeKey = 'cached_theme_mode';
  static const _publicNTFYKey = 'cached_public_notification_enabled';
  static const _chatNTFYKey = 'cached_chat_notification_enabled';
  static const _verifyNTFKey = 'cached_verified_users_only';

  final _localeNotifier = ValueNotifier<Locale?>(null);
  final _themeModeNotifier = ValueNotifier<ThemeMode>(ThemeMode.system);

  bool _publicTabNotificationsEnabled = true;
  bool _chatNotificationsEnabled = true;
  bool _notifyOnlyForVerifiedUsers = false;

  /// Initializes the singleton.
  ///
  /// If [prefs] is not provided, a default [SharedPreferencesAsync] is used.
  /// Returns immediately if already initialized.
  static Future<void> initialize({SharedPreferencesAsync? prefs}) async {
    if (_instance != null) return;

    _instance = UserPrefsHelper._internal(prefs ?? SharedPreferencesAsync());
    await _instance!._loadInitialValues();
  }

  /// Reload all internal state values from preferences.
  Future<void> refresh() => _loadInitialValues();

  Future<void> _loadInitialValues() async {
    _localeNotifier.value = await _loadLocaleFromPrefs();
    _themeModeNotifier.value = await _loadThemeModeFromPrefs();

    _publicTabNotificationsEnabled =
        await _prefs.getBool(_publicNTFYKey) ?? true;
    _chatNotificationsEnabled = await _prefs.getBool(_chatNTFYKey) ?? true;
    _notifyOnlyForVerifiedUsers = await _prefs.getBool(_verifyNTFKey) ?? false;

    await _cleanLegacyData();
  }

  /// Cleans up legacy data from previous app versions.
  ///
  /// This includes:
  /// - adaptive_theme SharedPreferences key
  /// - Hive box files (UserPreferencesBox.hive and .lock)
  ///
  /// Can be removed in a later release.
  Future<void> _cleanLegacyData() async {
    if (await _prefs.containsKey('adaptive_theme_preferences')) {
      await _prefs.remove('adaptive_theme_preferences');
    }
    final appDir = await getApplicationDocumentsDirectory();
    final boxName = 'UserPreferencesBox';
    for (final box in [boxName, boxName.toLowerCase()]) {
      final hiveBoxFile = File('${appDir.path}/$box.hive');
      final hiveLockFile = File('${appDir.path}/$box.lock');
      if (await hiveBoxFile.exists()) await hiveBoxFile.delete();
      if (await hiveLockFile.exists()) await hiveLockFile.delete();
    }
  }

  ValueListenable<Locale?> get localeNotifier => _localeNotifier;

  ValueListenable<ThemeMode> get themeModeNotifier => _themeModeNotifier;

  Future<Locale?> _loadLocaleFromPrefs() async {
    String? completeCode = await _prefs.getString(_defaultLocaleKey);
    if (completeCode == null) return null;
    final cs = completeCode.split('_');

    if (cs.last == 'null') return Locale.fromSubtags(languageCode: cs.first);
    return Locale.fromSubtags(languageCode: cs.first, countryCode: cs.last);
  }

  Future<ThemeMode> _loadThemeModeFromPrefs() async {
    final String? modeString = await _prefs.getString(_themeModeKey);

    if (modeString == null) {
      return ThemeMode.system;
    }

    switch (modeString.toLowerCase()) {
      case 'light':
        return ThemeMode.light;
      case 'dark':
        return ThemeMode.dark;
      case 'system':
      default:
        return ThemeMode.system;
    }
  }

  Locale? get defaultLocale => _localeNotifier.value;

  Future<void> setDefaultLocale(Locale? l) async {
    _localeNotifier.value = l;

    if (l == null) {
      await _prefs.remove(_defaultLocaleKey);
      return;
    }

    String code = '${l.languageCode}_${l.countryCode}';
    await _prefs.setString(_defaultLocaleKey, code);
  }

  ThemeMode get themeMode => _themeModeNotifier.value;

  Future<void> setThemeMode(ThemeMode mode) async {
    final modeString = mode == ThemeMode.light
        ? 'light'
        : mode == ThemeMode.dark
            ? 'dark'
            : 'system';
    await _prefs.setString(_themeModeKey, modeString);
    _themeModeNotifier.value = mode;
  }

  bool get publicTabNotificationsEnabled => _publicTabNotificationsEnabled;

  Future<void> setPublicTabNotificationsEnabled(bool val) async {
    await _prefs.setBool(_publicNTFYKey, val);
    _publicTabNotificationsEnabled = val;
  }

  bool get chatNotificationsEnabled => _chatNotificationsEnabled;

  Future<void> setChatNotificationsEnabled(bool val) async {
    await _prefs.setBool(_chatNTFYKey, val);
    _chatNotificationsEnabled = val;
  }

  bool get notifyOnlyForVerifiedUsers => _notifyOnlyForVerifiedUsers;

  Future<void> setNotifyOnlyForVerifiedUsers(bool val) async {
    await _prefs.setBool(_verifyNTFKey, val);
    _notifyOnlyForVerifiedUsers = val;
  }
}
