import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/providers/account_session_provider.dart';
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

  testWidgets(
    'does not redirect while refreshing a previous signed-in session',
    (tester) async {
      final refreshTriggerProvider = StateProvider((_) => false);
      final pendingRefresh = Completer<QaulAccountSessionState>();
      final container = ProviderContainer(
        overrides: [
          accountSessionProvider.overrideWith((ref) {
            final refreshing = ref.watch(refreshTriggerProvider);
            if (refreshing) return pendingRefresh.future;
            return QaulAccountSessionState.signedIn;
          }),
        ],
      );
      addTearDown(container.dispose);

      expect(
        await container.read(accountSessionProvider.future),
        QaulAccountSessionState.signedIn,
      );
      container.read(refreshTriggerProvider.notifier).state = true;
      container.read(accountSessionProvider);
      await Future<void>.delayed(Duration.zero);

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
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
    },
  );
}
