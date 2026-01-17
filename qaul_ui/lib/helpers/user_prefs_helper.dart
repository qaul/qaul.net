import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

class UserPrefsHelper {
  UserPrefsHelper._internal();
  
  static UserPrefsHelper? _instance;

  final _prefs = SharedPreferencesAsync();
  
  final _localeNotifier = ValueNotifier<Locale?>(null);
  final _themeModeNotifier = ValueNotifier<ThemeMode>(ThemeMode.system);
  final _publicTabNotificationsNotifier = ValueNotifier<bool>(true);
  final _chatNotificationsNotifier = ValueNotifier<bool>(true);
  final _verifiedUsersOnlyNotifier = ValueNotifier<bool>(false);

  static Future<void> initialize() async {
    if (_instance != null) return;
    
    _instance = UserPrefsHelper._internal();
    await _instance!._loadInitialValues();
  }
  
  Future<void> _loadInitialValues() async {
    _localeNotifier.value = await _loadLocaleFromPrefs();
    _themeModeNotifier.value = await _loadThemeModeFromPrefs();
    
    _publicTabNotificationsNotifier.value = await _prefs.getBool(_publicNTFYKey) ?? true;
    _chatNotificationsNotifier.value = await _prefs.getBool(_chatNTFYKey) ?? true;
    _verifiedUsersOnlyNotifier.value = await _prefs.getBool(_verifyNTFKey) ?? false;
    
    await _cleanLegacyAdaptiveThemeKey();
  }
  
  Future<void> _cleanLegacyAdaptiveThemeKey() async {
    try {
      final legacyPrefs = await SharedPreferences.getInstance();
      if (legacyPrefs.containsKey('adaptive_theme_preferences')) {
        await legacyPrefs.remove('adaptive_theme_preferences');
      }
    } catch (e) {
    }
  }
  
  @visibleForTesting
  static Future<void> resetForTesting() async {
    if (_instance != null) {
      await _instance!._prefs.clear();
    }
    _instance = null;
  }
  
  factory UserPrefsHelper() {
    assert(_instance != null, 'Call UserPrefsHelper.initialize() first!');
    return _instance!;
  }
  
  ValueListenable<Locale?> get localeNotifier => _localeNotifier;
  ValueListenable<ThemeMode> get themeModeNotifier => _themeModeNotifier;
  ValueListenable<bool> get publicTabNotificationsNotifier => 
      _publicTabNotificationsNotifier;
  ValueListenable<bool> get chatNotificationsNotifier => 
      _chatNotificationsNotifier;
  ValueListenable<bool> get verifiedUsersOnlyNotifier => 
      _verifiedUsersOnlyNotifier;

  String get _defaultLocaleKey => 'cached_default_locale';
  String get _themeModeKey => 'cached_theme_mode';
  String get _publicNTFYKey => 'cached_public_notification_enabled';

  String get _chatNTFYKey => 'cached_chat_notification_enabled';

  String get _verifyNTFKey => 'cached_verified_users_only';

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
    final modeString = mode == ThemeMode.light ? 'light' : mode == ThemeMode.dark ? 'dark' : 'system';
    await _prefs.setString(_themeModeKey, modeString);
    _themeModeNotifier.value = mode;
  }

  bool get publicTabNotificationsEnabled => 
      _publicTabNotificationsNotifier.value;

  Future<void> setPublicTabNotificationsEnabled(bool val) async {
    await _prefs.setBool(_publicNTFYKey, val);
    _publicTabNotificationsNotifier.value = val;
  }

  bool get chatNotificationsEnabled => _chatNotificationsNotifier.value;

  Future<void> setChatNotificationsEnabled(bool val) async {
    await _prefs.setBool(_chatNTFYKey, val);
    _chatNotificationsNotifier.value = val;
  }

  bool get notifyOnlyForVerifiedUsers => _verifiedUsersOnlyNotifier.value;

  Future<void> setNotifyOnlyForVerifiedUsers(bool v) async {
    await _prefs.setBool(_verifyNTFKey, v);
    _verifiedUsersOnlyNotifier.value = v;
  }
}
