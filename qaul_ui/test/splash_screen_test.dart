import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/providers/account_session_provider.dart';
import 'package:qaul_ui/screens/create_account_screen.dart';
import 'package:qaul_ui/screens/splash_screen.dart';

void main() {
  testWidgets(
    'keeps the splash loading state while redirecting signed in users',
    (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            accountSessionProvider.overrideWith(
              (ref) => QaulAccountSessionState.signedIn,
            ),
          ],
          child: MaterialApp(
            routes: {
              NavigationHelper.initial: (_) => const SplashScreen(),
              NavigationHelper.home: (_) => const Text('home'),
            },
          ),
        ),
      );

      expect(find.byType(QaulLoadingIndicator), findsOneWidget);
      expect(find.byKey(SplashScreen.createUserButtonKey), findsNothing);
    },
  );

  testWidgets('does not redirect while the session is loading', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          accountSessionProvider.overrideWithValue(
            const AsyncLoading<QaulAccountSessionState>(),
          ),
        ],
        child: MaterialApp(
          routes: {
            NavigationHelper.initial: (_) => const SplashScreen(),
            NavigationHelper.home: (_) => const Text('home'),
          },
        ),
      ),
    );

    await tester.pump();

    expect(find.byType(SplashScreen), findsOneWidget);
    expect(find.text('home'), findsNothing);
  });

  testWidgets('create account header back button returns to previous screen', (
    tester,
  ) async {
    const openCreateAccountKey = ValueKey('openCreateAccount');

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp(
          localizationsDelegates: [
            ...AppLocalizations.localizationsDelegates,
            QaulComponentsLocalizations.delegate,
          ],
          supportedLocales: AppLocalizations.supportedLocales,
          onGenerateRoute: NavigationHelper.onGenerateRoute,
          home: Builder(
            builder: (context) {
              return TextButton(
                key: openCreateAccountKey,
                onPressed: () => Navigator.pushNamed(
                  context,
                  NavigationHelper.createAccount,
                ),
                child: const Text('open'),
              );
            },
          ),
        ),
      ),
    );

    await tester.tap(find.byKey(openCreateAccountKey));
    await tester.pumpAndSettle();

    expect(find.byType(CreateAccountScreen), findsOneWidget);

    await tester.tap(find.byTooltip('Back'));
    await tester.pumpAndSettle();

    expect(find.byType(CreateAccountScreen), findsNothing);
    expect(find.byKey(openCreateAccountKey), findsOneWidget);
  });
}
