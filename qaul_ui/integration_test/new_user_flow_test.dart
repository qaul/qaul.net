import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:qaul_ui/main.dart' as app;
import 'package:qaul_ui/screens/create_account_screen.dart';
import 'package:qaul_ui/screens/home/home_screen.dart';
import 'package:qaul_ui/screens/splash_screen.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  Future<void> delay([int milliseconds = 250]) async =>
      await Future<void>.delayed(Duration(milliseconds: milliseconds));

  bool isPresent(Finder finder, WidgetTester tester) {
    try {
      return tester.any(finder);
    } catch (exception) {
      return false;
    }
  }

  testWidgets('test new user flow', (tester) async {
    final originalOnError = FlutterError.onError!;

    app.main();

    while (!isPresent(find.byKey(SplashScreen.widgetKey), tester)) {
      await tester.pump();
      await delay(10);
    }

    expect(find.byKey(SplashScreen.widgetKey), findsOneWidget);

    FlutterError.onError = (FlutterErrorDetails details) {
      originalOnError(details); // reinstating est framework's error handler
    };

    await tester.pumpAndSettle();
    expect(find.byKey(CreateAccountScreen.widgetKey), findsOneWidget);

    final usernameField = find.byType(TextFormField);
    await tester.enterText(usernameField, 'test');
    await tester.pump();

    await tester.tap(find.byKey(CreateAccountScreen.submitButtonKey));
    await tester.pumpAndSettle();

    await tester.tap(find.byKey(HomeScreen.widgetKey));
  });
}
