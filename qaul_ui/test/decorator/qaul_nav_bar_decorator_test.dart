import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/qaul_app.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'nav_bar_test_stubs.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final testUser = User(
    name: 'Test User',
    id: Uint8List.fromList('testUserId'.codeUnits),
  );

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  Widget buildDecorator() {
    return ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => testUser),
        publicNotificationControllerProvider.overrideWith(
          (ref) => StubPublicNotificationController(ref),
        ),
        chatNotificationControllerProvider.overrideWith(
          (ref) => StubChatNotificationController(ref),
        ),
      ],
      child: MaterialApp(
        theme: QaulApp.lightTheme,
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        home: Material(
          child: QaulNavBarDecorator(
            child: (pageViewKey) => SizedBox(key: pageViewKey),
          ),
        ),
      ),
    );
  }

  group('QaulNavBarDecorator', () {
    testWidgets('composes nav bar and content area', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      await tester.pumpWidget(buildDecorator());

      expect(find.byType(QaulNavBarWidget), findsOneWidget);
      expect(find.byType(Expanded), findsWidgets);
    });

    testWidgets('child builder is used and receives a key', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      GlobalKey? capturedKey;
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            defaultUserProvider.overrideWith((_) => testUser),
            publicNotificationControllerProvider.overrideWith(
              (ref) => StubPublicNotificationController(ref),
            ),
            chatNotificationControllerProvider.overrideWith(
              (ref) => StubChatNotificationController(ref),
            ),
          ],
          child: MaterialApp(
            theme: QaulApp.lightTheme,
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            home: Material(
              child: QaulNavBarDecorator(
                child: (key) {
                  capturedKey = key;
                  return SizedBox(key: key);
                },
              ),
            ),
          ),
        ),
      );

      expect(capturedKey, isNotNull);
      expect(find.byKey(capturedKey!), findsOneWidget);
    });

    testWidgets('decorator shows notification badge when public count is non-zero', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      late StubPublicNotificationController publicStub;
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            defaultUserProvider.overrideWith((_) => testUser),
            publicNotificationControllerProvider.overrideWith((ref) {
              publicStub = StubPublicNotificationController(ref);
              return publicStub;
            }),
            chatNotificationControllerProvider.overrideWith(
              (ref) => StubChatNotificationController(ref),
            ),
          ],
          child: MaterialApp(
            theme: QaulApp.lightTheme,
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            home: Material(
              child: QaulNavBarDecorator(
                child: (pageViewKey) => SizedBox(key: pageViewKey),
              ),
            ),
          ),
        ),
      );

      publicStub.newNotificationCount.value = 2;
      await tester.pump();

      expect(find.text('2'), findsOneWidget);
    });

    testWidgets('selecting overflow menu item navigates to route', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      const settingsKey = Key('settings_placeholder');
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            defaultUserProvider.overrideWith((_) => testUser),
            publicNotificationControllerProvider.overrideWith(
              (ref) => StubPublicNotificationController(ref),
            ),
            chatNotificationControllerProvider.overrideWith(
              (ref) => StubChatNotificationController(ref),
            ),
          ],
          child: MaterialApp(
            theme: QaulApp.lightTheme,
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            initialRoute: '/',
            routes: {
              '/': (_) => Material(
                child: QaulNavBarDecorator(
                  child: (key) => SizedBox(key: key),
                ),
              ),
              '/settings': (_) => Material(child: SizedBox(key: settingsKey)),
            },
          ),
        ),
      );

      await tester.tap(find.byWidgetPredicate((w) => w is PopupMenuButton));
      await tester.pumpAndSettle();

      await tester.tap(find.byWidgetPredicate(
        (w) => w is PopupMenuItem<NavBarOverflowOption> && w.value == NavBarOverflowOption.settings,
      ));
      await tester.pumpAndSettle();

      expect(find.byKey(settingsKey), findsOneWidget);
    });
  });
}
