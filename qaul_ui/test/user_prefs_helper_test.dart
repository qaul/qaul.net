import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() async {
    UserPrefsHelper.resetForTesting();
    SharedPreferencesAsyncPlatform.instance = InMemorySharedPreferencesAsync.empty();
    await UserPrefsHelper.initialize();
  });

  group('Hive to SharedPreferences migration', () {
    test('initializes with SharedPreferencesWithCache', () {
      expect(() => UserPrefsHelper(), returnsNormally);
    });

    test('returns same singleton instance', () {
      final helper1 = UserPrefsHelper();
      final helper2 = UserPrefsHelper();
      expect(identical(helper1, helper2), true);
    });

    test('reads and writes preferences correctly with updates', () {
      final helper = UserPrefsHelper();

      helper.defaultLocale = const Locale('pt', 'BR');
      expect(helper.defaultLocale, equals(const Locale('pt', 'BR')));

      helper.defaultLocale = const Locale('en', 'US');
      expect(helper.defaultLocale, equals(const Locale('en', 'US')));

      helper.publicTabNotificationsEnabled = false;
      expect(helper.publicTabNotificationsEnabled, false);

      helper.publicTabNotificationsEnabled = true;
      expect(helper.publicTabNotificationsEnabled, true);
    });

    test('notifies listeners when preferences change', () {
      final helper = UserPrefsHelper();
      int localeChanges = 0;
      int chatNotifChanges = 0;

      helper.localeNotifier.addListener(() {
        localeChanges++;
      });
      
      helper.chatNotificationsNotifier.addListener(() {
        chatNotifChanges++;
      });

      helper.defaultLocale = const Locale('en', 'US');
      expect(localeChanges, 1);
      expect(chatNotifChanges, 0);

      helper.chatNotificationsEnabled = false;
      expect(localeChanges, 1);
      expect(chatNotifChanges, 1);
    });

    test('uses correct default values', () {
      final helper = UserPrefsHelper();

      expect(helper.defaultLocale, isNull);
      expect(helper.publicTabNotificationsEnabled, true);
      expect(helper.chatNotificationsEnabled, true);
      expect(helper.notifyOnlyForVerifiedUsers, false);
    });
  });
}
