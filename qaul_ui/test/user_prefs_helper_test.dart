import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  late SharedPreferencesAsync testPrefs;

  setUpAll(() async {
    SharedPreferencesAsyncPlatform.instance =
        InMemorySharedPreferencesAsync.empty();
    testPrefs = SharedPreferencesAsync();
    await UserPrefsHelper.initialize(prefs: testPrefs);
  });

  setUp(() async {
    await testPrefs.clear();
    await UserPrefsHelper.instance.refresh();
  });

  group('UserPrefsHelper', () {
    test('initializes with SharedPreferencesWithCache', () {
      expect(() => UserPrefsHelper.instance, returnsNormally);
    });

    test('returns same singleton instance', () {
      final helper1 = UserPrefsHelper.instance;
      final helper2 = UserPrefsHelper.instance;
      expect(identical(helper1, helper2), true);
    });

    test('reads and writes preferences correctly with updates', () async {
      final helper = UserPrefsHelper.instance;

      await helper.setDefaultLocale(const Locale('pt', 'BR'));
      expect(helper.defaultLocale, equals(const Locale('pt', 'BR')));

      await helper.setDefaultLocale(const Locale('en', 'US'));
      expect(helper.defaultLocale, equals(const Locale('en', 'US')));

      await helper.setPublicTabNotificationsEnabled(false);
      expect(helper.publicTabNotificationsEnabled, false);

      await helper.setPublicTabNotificationsEnabled(true);
      expect(helper.publicTabNotificationsEnabled, true);
    });

    test('notifies listeners when preferences change', () async {
      final helper = UserPrefsHelper.instance;
      int localeChanges = 0;
      int themeChanges = 0;

      helper.localeNotifier.addListener(() {
        localeChanges++;
      });

      helper.themeModeNotifier.addListener(() {
        themeChanges++;
      });

      await helper.setDefaultLocale(const Locale('en', 'US'));
      expect(localeChanges, 1);
      expect(themeChanges, 0);

      await helper.setThemeMode(ThemeMode.dark);
      expect(localeChanges, 1);
      expect(themeChanges, 1);
    });

    test('uses correct default values', () {
      final helper = UserPrefsHelper.instance;

      expect(helper.defaultLocale, isNull);
      expect(helper.themeMode, ThemeMode.system);
      expect(helper.publicTabNotificationsEnabled, true);
      expect(helper.chatNotificationsEnabled, true);
      expect(helper.notifyOnlyForVerifiedUsers, false);
    });

    test('reads and writes theme mode correctly', () async {
      final helper = UserPrefsHelper.instance;

      await helper.setThemeMode(ThemeMode.dark);
      expect(helper.themeMode, equals(ThemeMode.dark));

      await helper.setThemeMode(ThemeMode.light);
      expect(helper.themeMode, equals(ThemeMode.light));

      await helper.setThemeMode(ThemeMode.system);
      expect(helper.themeMode, equals(ThemeMode.system));
    });
  });
}
