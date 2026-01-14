import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

class UserPrefsHelper {
  UserPrefsHelper._internal(this._prefs) {
    _localeNotifier = ValueNotifier(_loadLocaleFromPrefs());
    _publicTabNotificationsNotifier = _createBoolNotifier(_publicNTFYKey, defaultValue: true);
    _chatNotificationsNotifier = _createBoolNotifier(_chatNTFYKey, defaultValue: true);
    _verifiedUsersOnlyNotifier = _createBoolNotifier(_verifyNTFKey, defaultValue: false);
  }
  
  ValueNotifier<bool> _createBoolNotifier(String key, {required bool defaultValue}) {
    return ValueNotifier(_prefs.getBool(key) ?? defaultValue);
  }
  
  static UserPrefsHelper? _instance;
  static final Completer<void> _readyCompleter = Completer();
  
  final SharedPreferencesWithCache _prefs;
  
  late final ValueNotifier<Locale?> _localeNotifier;
  late final ValueNotifier<bool> _publicTabNotificationsNotifier;
  late final ValueNotifier<bool> _chatNotificationsNotifier;
  late final ValueNotifier<bool> _verifiedUsersOnlyNotifier;
  
  static Future<void> get ready => _readyCompleter.future;
  
  static Future<void> initialize() async {
    if (_instance != null) return;
    
    final prefs = await SharedPreferencesWithCache.create(
      cacheOptions: const SharedPreferencesWithCacheOptions(),
    );
    _instance = UserPrefsHelper._internal(prefs);
    
    if (!_readyCompleter.isCompleted) {
      _readyCompleter.complete();
    }
  }
  
  @visibleForTesting
  static void resetForTesting() {
    _instance = null;
  }
  
  factory UserPrefsHelper() {
    assert(_instance != null, 'Call UserPrefsHelper.initialize() first!');
    return _instance!;
  }
  
  ValueListenable<Locale?> get localeNotifier => _localeNotifier;
  ValueListenable<bool> get publicTabNotificationsNotifier => 
      _publicTabNotificationsNotifier;
  ValueListenable<bool> get chatNotificationsNotifier => 
      _chatNotificationsNotifier;
  ValueListenable<bool> get verifiedUsersOnlyNotifier => 
      _verifiedUsersOnlyNotifier;

  String get _defaultLocaleKey => 'cached_default_locale';

  String get _publicNTFYKey => 'cached_public_notification_enabled';

  String get _chatNTFYKey => 'cached_chat_notification_enabled';

  String get _verifyNTFKey => 'cached_verified_users_only';

  Locale? _loadLocaleFromPrefs() {
    String? completeCode = _prefs.getString(_defaultLocaleKey);
    if (completeCode == null) return null;
    final cs = completeCode.split('_');

    if (cs.last == 'null') return Locale.fromSubtags(languageCode: cs.first);
    return Locale.fromSubtags(languageCode: cs.first, countryCode: cs.last);
  }

  Locale? get defaultLocale => _localeNotifier.value;

  set defaultLocale(Locale? l) {
    if (l == null) {
      _prefs.remove(_defaultLocaleKey);
    } else {
      String code = '${l.languageCode}_${l.countryCode}';
      _prefs.setString(_defaultLocaleKey, code);
    }
    _localeNotifier.value = l;
  }

  bool get publicTabNotificationsEnabled => 
      _publicTabNotificationsNotifier.value;

  set publicTabNotificationsEnabled(bool val) {
    _prefs.setBool(_publicNTFYKey, val);
    _publicTabNotificationsNotifier.value = val;
  }

  bool get chatNotificationsEnabled => _chatNotificationsNotifier.value;

  set chatNotificationsEnabled(bool val) {
    _prefs.setBool(_chatNTFYKey, val);
    _chatNotificationsNotifier.value = val;
  }

  bool get notifyOnlyForVerifiedUsers => _verifiedUsersOnlyNotifier.value;

  set notifyOnlyForVerifiedUsers(bool v) {
    _prefs.setBool(_verifyNTFKey, v);
    _verifiedUsersOnlyNotifier.value = v;
  }
}
