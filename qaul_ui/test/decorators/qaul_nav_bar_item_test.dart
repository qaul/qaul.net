import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/nav_bar/widgets/qaul_nav_bar_decorator.dart';
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

  Widget wrapNavBarItem(TabType tab) {
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
          child: QaulNavBarItem(tab),
        ),
      ),
    );
  }

  group('QaulNavBarItem', () {
    testWidgets('account tab shows avatar', (tester) async {
      await tester.pumpWidget(wrapNavBarItem(TabType.account));

      expect(find.byType(CircleAvatar), findsOneWidget);
    });

    testWidgets('public tab renders', (tester) async {
      await tester.pumpWidget(wrapNavBarItem(TabType.public));

      expect(find.byType(QaulNavBarItem), findsOneWidget);
    });

    testWidgets('users tab renders', (tester) async {
      await tester.pumpWidget(wrapNavBarItem(TabType.users));

      expect(find.byType(QaulNavBarItem), findsOneWidget);
    });

    testWidgets('chat tab renders', (tester) async {
      await tester.pumpWidget(wrapNavBarItem(TabType.chat));

      expect(find.byType(QaulNavBarItem), findsOneWidget);
    });

    testWidgets('network tab renders', (tester) async {
      await tester.pumpWidget(wrapNavBarItem(TabType.network));

      expect(find.byType(QaulNavBarItem), findsOneWidget);
    });

    testWidgets('tapping account tab switches to account', (tester) async {
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
              child: Column(
                children: [
                  QaulNavBarItem(TabType.account),
                  Expanded(
                    child: Consumer(
                      builder: (context, ref, _) {
                        final currentTab =
                            ref.watch(homeScreenControllerProvider);
                        return Text('current:$currentTab');
                      },
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );

      expect(find.text('current:TabType.public'), findsOneWidget);

      await tester.tap(find.byType(QaulNavBarItem));
      await tester.pump();

      expect(find.text('current:TabType.account'), findsOneWidget);
    });
  });
}
