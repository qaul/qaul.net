import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/qaul_app.dart';

void main() {
  test('system bar styles match the selected brightness', () {
    final lightStyle = QaulApp.systemUiOverlayStyleFor(Brightness.light);
    final darkStyle = QaulApp.systemUiOverlayStyleFor(Brightness.dark);

    expect(lightStyle.statusBarColor, Colors.white);
    expect(lightStyle.systemNavigationBarColor, Colors.white);
    expect(lightStyle.statusBarIconBrightness, Brightness.dark);
    expect(lightStyle.systemNavigationBarIconBrightness, Brightness.dark);
    expect(lightStyle.systemStatusBarContrastEnforced, isFalse);
    expect(lightStyle.systemNavigationBarContrastEnforced, isFalse);

    expect(darkStyle.statusBarColor, Colors.black);
    expect(darkStyle.systemNavigationBarColor, Colors.black);
    expect(darkStyle.statusBarIconBrightness, Brightness.light);
    expect(darkStyle.systemNavigationBarIconBrightness, Brightness.light);
    expect(darkStyle.systemStatusBarContrastEnforced, isFalse);
    expect(darkStyle.systemNavigationBarContrastEnforced, isFalse);
  });

  test('app bars use the app-wide system bar styles', () {
    expect(
      QaulApp.lightTheme.appBarTheme.systemOverlayStyle,
      QaulApp.systemUiOverlayStyleFor(Brightness.light),
    );
    expect(
      QaulApp.darkTheme.appBarTheme.systemOverlayStyle,
      QaulApp.systemUiOverlayStyleFor(Brightness.dark),
    );
  });
}
