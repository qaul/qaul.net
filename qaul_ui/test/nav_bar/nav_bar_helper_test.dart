import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/nav_bar/nav_bar_helper.dart';
import 'package:qaul_ui/qaul_app.dart';

void main() {
  group('NavBarOverflowOption', () {
    test('has six values', () {
      expect(NavBarOverflowOption.values.length, 6);
    });

    test('contains all expected options', () {
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.settings));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.about));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.license));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.support));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.oldNetwork));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.files));
    });
  });

  group('navBarOverflowMenuLabels', () {
    testWidgets('returns map with all six options for en locale', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          theme: QaulApp.lightTheme,
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          locale: const Locale('en'),
          home: Builder(
            builder: (context) {
              final labels = navBarOverflowMenuLabels(context);
              expect(labels.length, 6);
              expect(labels[NavBarOverflowOption.settings], isNotEmpty);
              expect(labels[NavBarOverflowOption.about], isNotEmpty);
              expect(labels[NavBarOverflowOption.license], isNotEmpty);
              expect(labels[NavBarOverflowOption.support], isNotEmpty);
              expect(labels[NavBarOverflowOption.oldNetwork], isNotEmpty);
              expect(labels[NavBarOverflowOption.files], isNotEmpty);
              return const SizedBox.shrink();
            },
          ),
        ),
      );
    });
  });

  group('handleNavBarOverflowSelected', () {
    testWidgets('navigates to settings when settings option is selected', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          theme: QaulApp.lightTheme,
          initialRoute: '/',
          routes: {
            '/': (_) => Builder(
              builder: (context) => ElevatedButton(
                onPressed: () => handleNavBarOverflowSelected(context, NavBarOverflowOption.settings),
                child: const Text('Go'),
              ),
            ),
            '/settings': (_) => const Scaffold(body: Text('Settings')),
          },
        ),
      );

      await tester.tap(find.text('Go'));
      await tester.pumpAndSettle();

      expect(find.text('Settings'), findsOneWidget);
    });
  });
}
