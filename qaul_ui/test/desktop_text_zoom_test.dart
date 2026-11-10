// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/widgets/desktop_text_zoom.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

class _DocumentsPath extends Fake with MockPlatformInterfaceMixin
    implements PathProviderPlatform {
  _DocumentsPath(this.path);
  final String path;

  @override
  Future<String?> getApplicationDocumentsPath() async => path;
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Directory documents;

  setUpAll(() async {
    documents = await Directory.systemTemp.createTemp('text_zoom_test_');
    PathProviderPlatform.instance = _DocumentsPath(documents.path);
    SharedPreferencesAsyncPlatform.instance = InMemorySharedPreferencesAsync.empty();
    await UserPrefsHelper.initialize(prefs: SharedPreferencesAsync());
  });

  tearDownAll(() async => documents.delete(recursive: true));
  setUp(() async => UserPrefsHelper.instance.setTextScale(1));

  Future<void> press(
    WidgetTester tester,
    TargetPlatform platform,
    LogicalKeyboardKey key, {
    bool shift = false,
  }) async {
    final modifier = platform == TargetPlatform.macOS
        ? LogicalKeyboardKey.metaLeft
        : LogicalKeyboardKey.controlLeft;
    await tester.sendKeyDownEvent(modifier);
    if (shift) await tester.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
    await tester.sendKeyEvent(
      key,
      physicalKey: key == LogicalKeyboardKey.add ? PhysicalKeyboardKey.equal : null,
    );
    if (shift) await tester.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
    await tester.sendKeyUpEvent(modifier);
    await tester.pump();
  }

  for (final platform in TargetPlatform.values) {
    testWidgets('text zoom shortcuts on $platform with the editor focused', (tester) async {
      final controller = TextEditingController(text: 'Unsent draft');
      addTearDown(controller.dispose);
      await tester.pumpWidget(MaterialApp(
        theme: ThemeData(platform: platform),
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        builder: (context, child) => DesktopTextZoom(child: child!),
        home: Scaffold(body: TextField(controller: controller)),
      ));
      await tester.tap(find.byType(TextField));
      await tester.pump();
      final popup = find.byKey(const ValueKey('desktop-text-zoom-popup'));
      bool popupVisible() =>
          popup.evaluate().isNotEmpty && tester.widget<AnimatedOpacity>(popup).opacity == 1;
      expect(popupVisible(), isFalse);

      double effectiveScale() => MediaQuery.textScalerOf(
        tester.element(find.byType(TextField)),
      ).scale(16) / 16;

      final desktop = platform == TargetPlatform.macOS ||
          platform == TargetPlatform.windows || platform == TargetPlatform.linux;
      if (!desktop) {
        await tester.runAsync(() => UserPrefsHelper.instance.setTextScale(2.5));
        await tester.pump();
        expect(effectiveScale(), 1);
        expect(popupVisible(), isFalse);
        await press(tester, platform, LogicalKeyboardKey.minus);
        expect(UserPrefsHelper.instance.textScale, 2.5);
        return;
      }

      await press(tester, platform, LogicalKeyboardKey.equal);
      expect(effectiveScale(), 1.1);
      expect(popupVisible(), isTrue);
      expect(find.text('110%'), findsOneWidget);
      await press(tester, platform, LogicalKeyboardKey.equal, shift: true);
      expect(effectiveScale(), 1.2);
      await press(tester, platform, LogicalKeyboardKey.add);
      expect(effectiveScale(), 1.3);
      await press(tester, platform, LogicalKeyboardKey.minus);
      expect(effectiveScale(), 1.2);
      await press(tester, platform, LogicalKeyboardKey.digit0);
      expect(effectiveScale(), 1);
      expect(popupVisible(), isFalse);
      for (var i = 0; i < 20; i++) {
        await press(tester, platform, LogicalKeyboardKey.numpadAdd);
      }
      expect(effectiveScale(), 2.5);
      for (var i = 0; i < 25; i++) {
        await press(tester, platform, LogicalKeyboardKey.numpadSubtract);
      }
      expect(effectiveScale(), 0.6);
      expect(popupVisible(), isTrue);
      expect(find.text('60%'), findsOneWidget);
      await tester.pump(DesktopTextZoom.popupDuration);
      expect(popupVisible(), isFalse);
      expect(effectiveScale(), 0.6);
      await press(tester, platform, LogicalKeyboardKey.equal);
      expect(find.text('70%'), findsOneWidget);
      await tester.pump(DesktopTextZoom.popupDuration - const Duration(milliseconds: 500));
      await press(tester, platform, LogicalKeyboardKey.minus);
      await tester.pump(const Duration(seconds: 2));
      expect(find.text('60%'), findsOneWidget);
      await tester.tap(find.text('Reset to default'));
      await tester.pump();
      expect(effectiveScale(), 1);
      expect(popupVisible(), isFalse);
      await press(tester, platform, LogicalKeyboardKey.equal);
      await press(tester, platform, LogicalKeyboardKey.numpad0);
      expect(effectiveScale(), 1);
      expect(controller.text, 'Unsent draft');
      expect(tester.takeException(), isNull);
    });
  }
}
