import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/decorators/qaul_navbar_decorator.dart';
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

  Widget buildDecoratorWithPadding({
    required EdgeInsets systemPadding,
    required Size size,
    void Function(EdgeInsets childPadding)? onChildBuilt,
  }) {
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
      child: MediaQuery(
        data: MediaQueryData(size: size, padding: systemPadding),
        child: MaterialApp(
          theme: QaulApp.lightTheme,
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          builder: (context, child) => child!,
          home: Material(
            child: QaulNavBarDecorator(
              child: (pageViewKey) => Builder(
                builder: (context) {
                  onChildBuilt?.call(MediaQuery.paddingOf(context));
                  return SizedBox(key: pageViewKey);
                },
              ),
            ),
          ),
        ),
      ),
    );
  }

  group('tablet layout SafeArea', () {
    testWidgets('has SafeArea with left:false and bottom:false',
        (tester) async {
      await tester.binding.setSurfaceSize(const Size(1024, 768));
      await tester.pumpWidget(buildDecorator());

      final safeAreas = tester.widgetList<SafeArea>(find.byType(SafeArea));
      final tabletSafeArea = safeAreas.where(
        (sa) => sa.left == false && sa.bottom == false,
      );
      expect(tabletSafeArea, isNotEmpty,
          reason: 'tablet body should be wrapped in '
              'SafeArea(left: false, bottom: false)');
    });

    testWidgets('consumes right system inset for content pane',
        (tester) async {
      await tester.binding.setSurfaceSize(const Size(1024, 768));

      EdgeInsets? observed;
      await tester.pumpWidget(buildDecoratorWithPadding(
        size: const Size(1024, 768),
        systemPadding: const EdgeInsets.only(right: 48),
        onChildBuilt: (p) => observed = p,
      ));

      expect(observed, isNotNull);
      expect(observed!.right, 0.0,
          reason: 'SafeArea should consume the right inset');
    });

    testWidgets('preserves left padding for descendants (no double-inset)',
        (tester) async {
      await tester.binding.setSurfaceSize(const Size(1024, 768));

      EdgeInsets? observed;
      await tester.pumpWidget(buildDecoratorWithPadding(
        size: const Size(1024, 768),
        systemPadding: const EdgeInsets.only(left: 24),
        onChildBuilt: (p) => observed = p,
      ));

      expect(observed, isNotNull);
      expect(observed!.left, 24.0,
          reason: 'left padding should pass through since '
              'SafeArea has left:false — navbar handles this edge');
    });
  });

  group('QaulNavBarDecorator', () {
    testWidgets('composes nav bar and content area', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      await tester.pumpWidget(buildDecorator());

      expect(find.byType(QaulNavBar), findsOneWidget);
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

      final overflowInk = find.descendant(
        of: find.byType(QaulNavBar),
        matching: find.byWidgetPredicate(
          (w) => w is InkWell && w.onTap == null && w.onTapDown != null,
        ),
      );
      expect(overflowInk, findsOneWidget);
      await tester.tap(overflowInk);
      await tester.pumpAndSettle();

      await tester.tap(find.byWidgetPredicate(
        (w) => w is PopupMenuItem<NavBarOverflowOption> && w.value == NavBarOverflowOption.settings,
      ));
      await tester.pumpAndSettle();

      expect(find.byKey(settingsKey), findsOneWidget);
    });
  });
}
