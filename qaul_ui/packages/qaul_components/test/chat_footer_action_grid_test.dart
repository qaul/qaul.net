import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

const _kGoldenKey = ValueKey('chat-footer-golden');
const _kPillKey = ValueKey('chat-footer-composer-pill');
const _kVoiceKey = ValueKey('chat-footer-voice');
const _kCameraKey = ValueKey('chat-footer-camera');
const _kMoreKey = ValueKey('chat-footer-more');
const _kSendKey = ValueKey('chat-footer-send');

void main() {
  Future<void> pumpFooter(
    WidgetTester tester, {
    required double width,
    required Brightness brightness,
    String text = '',
    required bool includeVoiceAndCamera,
  }) async {
    final controller = TextEditingController(text: text);
    addTearDown(controller.dispose);
    tester.view.physicalSize = Size(width, 180);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(
      MaterialApp(
        theme: ThemeData(brightness: brightness),
        home: Scaffold(
          body: RepaintBoundary(
            key: _kGoldenKey,
            child: Align(
              alignment: Alignment.bottomCenter,
              child: ChatFooter(
                placeholder: 'Private message',
                controller: controller,
                onSend: (_) {},
                onVoicePressed: includeVoiceAndCamera ? () {} : null,
                onCameraPressed: includeVoiceAndCamera ? () {} : null,
                onMoreAttachmentsPressed: () {},
                applyBottomSafeArea: false,
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
  }

  void expectTrailingGrid(
    WidgetTester tester,
    List<Key> keys, {
    required double expectedPillWidth,
  }) {
    final pill = tester.getRect(find.byKey(_kPillKey));
    final actionRects = [
      for (final key in keys) tester.getRect(find.byKey(key)),
    ];

    expect(pill.width, closeTo(expectedPillWidth, 1));
    expect(actionRects.last.right, closeTo(pill.right, 1));
    for (final rect in actionRects) {
      expect(rect.width, greaterThanOrEqualTo(44));
      expect(rect.height, greaterThanOrEqualTo(44));
    }
    for (var index = 1; index < actionRects.length; index++) {
      expect(
        actionRects[index].center.dx - actionRects[index - 1].center.dx,
        closeTo(44, 1),
      );
    }
  }

  testWidgets('anchors empty and typing action grids at phone widths', (
    tester,
  ) async {
    for (final width in [360.0, 390.0]) {
      for (final brightness in Brightness.values) {
        await pumpFooter(
          tester,
          width: width,
          brightness: brightness,
          includeVoiceAndCamera: true,
        );
        expectTrailingGrid(tester, [
          _kVoiceKey,
          _kCameraKey,
          _kMoreKey,
        ], expectedPillWidth: width - 40);

        await pumpFooter(
          tester,
          width: width,
          brightness: brightness,
          text: 'A message',
          includeVoiceAndCamera: true,
        );
        expectTrailingGrid(tester, [
          _kMoreKey,
          _kSendKey,
        ], expectedPillWidth: width - 40);

        await pumpFooter(
          tester,
          width: width,
          brightness: brightness,
          text: 'A message that wraps onto\na second line',
          includeVoiceAndCamera: true,
        );
        expectTrailingGrid(tester, [
          _kMoreKey,
          _kSendKey,
        ], expectedPillWidth: width - 40);
      }
    }
  });

  for (final width in [360, 390]) {
    for (final brightness in Brightness.values) {
      testWidgets('matches $width px ${brightness.name} empty-input golden', (
        tester,
      ) async {
        await pumpFooter(
          tester,
          width: width.toDouble(),
          brightness: brightness,
          includeVoiceAndCamera: true,
        );

        await expectLater(
          find.byKey(_kGoldenKey),
          matchesGoldenFile(
            'goldens/chat_footer_empty_${brightness.name}_$width.png',
          ),
        );
      });
    }
  }
}
