import 'package:flutter/material.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart'
    show MessageStatus, MessageType, QaulChatBubble, QaulChatBubbleMessage;
import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';

// flutter_chat_ui's messageWidth on a 390px-wide phone; the bubble caps at 272.
const _messageWidth = 304;
const _maxBubbleWidth = 272.0;

void main() {
  final author = types.User(id: 'peer');

  Future<void> pumpInBubble(
    WidgetTester tester,
    Widget media, {
    TextScaler textScaler = TextScaler.noScaling,
  }) {
    final clock = DateTime(2026, 1, 1, 12);
    return tester.pumpWidget(
      MaterialApp(
        home: MediaQuery(
          data: MediaQueryData(
            size: const Size(390, 844),
            textScaler: textScaler,
          ),
          child: Material(
            child: QaulChatBubble.media(
              message: QaulChatBubbleMessage(
                content: '',
                sentAt: clock,
                receivedAt: clock,
                status: MessageStatus.sent,
                messageType: MessageType.secondary,
                edges: const [],
              ),
              clock: clock,
              child: media,
            ),
          ),
        ),
      ),
    );
  }

  double bubbleWidth(WidgetTester tester) => tester
      .getRect(find.byKey(const ValueKey('chat-bubble-surface')))
      .width;

  types.ImageMessage image(double width, double height, {String? caption}) =>
      types.ImageMessage(
        id: 'image-$width-$height',
        author: author,
        name: 'photo.jpg',
        size: 1,
        uri: '/tmp/does-not-exist.jpg',
        width: width,
        height: height,
        metadata: caption == null ? null : {'description': caption},
      );

  for (final c in [
    (name: 'portrait 3:4', w: 3000.0, h: 4000.0, expected: 228.0),
    (name: 'portrait 9:16', w: 1080.0, h: 1920.0, expected: 171.0),
    (name: 'small portrait thumbnail', w: 120.0, h: 160.0, expected: 228.0),
    (name: 'landscape 4:3', w: 4000.0, h: 3000.0, expected: _maxBubbleWidth),
    (name: 'square', w: 1000.0, h: 1000.0, expected: _maxBubbleWidth),
  ]) {
    testWidgets('image bubble hugs a ${c.name} image', (tester) async {
      await pumpInBubble(
        tester,
        ImageMessageWidget(
          message: image(c.w, c.h),
          messageWidth: _messageWidth,
        ),
      );
      expect(tester.takeException(), isNull);
      expect(bubbleWidth(tester), closeTo(c.expected, 0.5));
      expect(
        tester.getRect(find.byType(Image)).width,
        closeTo(c.expected, 0.5),
      );
    });
  }

  testWidgets('image with a long caption widens to the bubble max', (
    tester,
  ) async {
    await pumpInBubble(
      tester,
      ImageMessageWidget(
        message: image(
          3000,
          4000,
          caption: 'A caption long enough to need the whole bubble width',
        ),
        messageWidth: _messageWidth,
      ),
    );
    expect(tester.takeException(), isNull);
    expect(bubbleWidth(tester), _maxBubbleWidth);
  });

  testWidgets('file bubble keeps the full bubble width', (tester) async {
    await pumpInBubble(
      tester,
      SizedBox(
        width: _messageWidth.toDouble(),
        child: FileMessageWidget(
          message: types.FileMessage(
            id: 'file',
            author: author,
            name: 'document.pdf',
            size: 1,
            uri: '/tmp/document.pdf',
          ),
        ),
      ),
    );
    expect(tester.takeException(), isNull);
    expect(bubbleWidth(tester), _maxBubbleWidth);
  });

  for (final scale in [1.0, 2.5]) {
    testWidgets('file bubble shortens a long file name (${scale}x)', (
      tester,
    ) async {
      final name = '${'a' * 116}.pdf';
      await pumpInBubble(
        tester,
        SizedBox(
          width: _messageWidth.toDouble(),
          child: FileMessageWidget(
            message: types.FileMessage(
              id: 'file',
              author: author,
              name: name,
              size: 1,
              uri: '/tmp/document.pdf',
            ),
          ),
        ),
        textScaler: TextScaler.linear(scale),
      );
      expect(tester.takeException(), isNull);
      expect(bubbleWidth(tester), _maxBubbleWidth);
      final text = tester.widget<Text>(
        find.byWidgetPredicate(
          (widget) => widget is Text && widget.semanticsLabel == name,
        ),
      );
      expect(text.maxLines, 1);
      // The test font draws every glyph 1em wide, so at 2.5x only the
      // extension fits; real fonts keep part of the name.
      if (scale == 1) expect(text.data, contains('…'));
      expect(text.data, endsWith('.pdf'));
    });
  }
}
