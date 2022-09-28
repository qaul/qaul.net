import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:qaul_ui/main.dart' as app;
import 'package:qaul_ui/screens/create_account_screen.dart';
import 'package:qaul_ui/screens/home/home_screen.dart';
import 'package:qaul_ui/screens/splash_screen.dart';

import 'src/screenshot_comparator.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  String? goldenSuffix;
  if (Platform.isIOS) goldenSuffix = '-ios';

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

    final createUserButtonFinder = find.byKey(SplashScreen.createUserButtonKey);
    while (!isPresent(createUserButtonFinder, tester)) {
      await tester.pump();
      await delay(10);
    }

    expect(createUserButtonFinder, findsOneWidget);

    FlutterError.onError = (FlutterErrorDetails details) {
      originalOnError(details); // reinstating est framework's error handler
    };

    // This is required prior to taking the screenshot (Android only).
    await binding.convertFlutterSurfaceToImage();

    if (goldenSuffix != null) {
      var bytes = await binding.takeScreenshot('screenshot');
      await expectGoldenMatches(bytes, 'splashScreenGolden$goldenSuffix.png');
    }

    await tester.tap(createUserButtonFinder);

    await tester.pump();
    await tester.pumpAndSettle();
    expect(find.byKey(CreateAccountScreen.widgetKey), findsOneWidget);

    if (goldenSuffix != null) {
      var bytes = await binding.takeScreenshot('screenshot');
      await expectGoldenMatches(bytes, 'createAccountGolden$goldenSuffix.png');
    }

    final usernameField = find.byType(TextFormField);
    await tester.enterText(usernameField, 'test');
    await tester.pump();

    await tester.tap(find.byKey(CreateAccountScreen.submitButtonKey));
    await tester.pumpAndSettle();

    expect(find.text('An error occurred'), findsNothing);

    await tester.tap(find.byKey(HomeScreen.widgetKey));

    if (goldenSuffix != null) {
      var bytes = await binding.takeScreenshot('screenshot');
      await expectGoldenMatches(bytes, 'homeScreenGolden$goldenSuffix.png');
    }
  });
}
