import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/nav_bar/nav_bar_helper.dart';
import 'package:qaul_ui/nav_bar/widgets/qaul_nav_bar.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/qaul_app.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../nav_bar_test_stubs.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final testUser = User(
    name: 'Test User',
    id: Uint8List.fromList('testUserId'.codeUnits),
  );

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  Map<NavBarOverflowOption, String> stubOverflowLabels() => {
        NavBarOverflowOption.settings: 'Settings',
        NavBarOverflowOption.about: 'About',
        NavBarOverflowOption.license: 'License',
        NavBarOverflowOption.support: 'Support',
        NavBarOverflowOption.oldNetwork: 'Routing',
        NavBarOverflowOption.files: 'Files',
      };

  Widget buildNavBar({required bool vertical}) {
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
          child: QaulNavBar(
            vertical: vertical,
            overflowMenuLabels: stubOverflowLabels(),
            onOverflowSelected: (_) {},
          ),
        ),
      ),
    );
  }

  group('QaulNavBar', () {
    testWidgets('shows five tab items', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      await tester.pumpWidget(buildNavBar(vertical: false));

      expect(find.byType(QaulNavBarItem), findsNWidgets(5));
    });

    testWidgets('shows overflow menu', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      await tester.pumpWidget(buildNavBar(vertical: false));

      expect(find.byWidgetPredicate((w) => w is PopupMenuButton), findsOneWidget);
    });

    testWidgets('overflow menu shows six options when opened', (tester) async {
      await tester.binding.setSurfaceSize(const Size(400, 800));
      await tester.pumpWidget(buildNavBar(vertical: false));

      await tester.tap(find.byWidgetPredicate((w) => w is PopupMenuButton));
      await tester.pumpAndSettle();

      expect(find.byWidgetPredicate((w) => w is PopupMenuItem), findsNWidgets(6));
    });

    testWidgets('invokes onOverflowSelected when menu item is tapped', (tester) async {
      NavBarOverflowOption? selected;
      await tester.binding.setSurfaceSize(const Size(400, 800));
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
              child: QaulNavBar(
                vertical: false,
                overflowMenuLabels: stubOverflowLabels(),
                onOverflowSelected: (option) => selected = option,
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.byWidgetPredicate((w) => w is PopupMenuButton));
      await tester.pumpAndSettle();
      await tester.tap(find.text('About'));
      await tester.pumpAndSettle();

      expect(selected, NavBarOverflowOption.about);
    });
  });
}
