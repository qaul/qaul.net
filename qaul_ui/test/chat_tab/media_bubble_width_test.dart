import 'package:flutter/material.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart'
    show
        ChatBubbleStyle,
        MessageStatus,
        MessageType,
        QaulChatBubble,
        QaulChatBubbleMessage;
import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';

// A 390px phone column. Bubbles cap at 76% of it, and chat.dart builds media
// at that content width.
const _column = 390.0;
const _maxBubbleWidth = _column * ChatBubbleStyle.maxWidthFraction;
final _messageWidth = _maxBubbleWidth.floor();

void main() {
  final author = types.User(id: 'peer');

  Future<void> pumpInBubble(
    WidgetTester tester,
    Widget media, {
    TextScaler textScaler = TextScaler.noScaling,
    double column = _column,
  }) {
    tester.view.physicalSize = Size(column, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);
    final clock = DateTime(2026, 1, 1, 12);
    return tester.pumpWidget(
      MaterialApp(
        home: MediaQuery(
          data: MediaQueryData(
            size: Size(column, 844),
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

  // Images are as wide as their aspect ratio allows at a height of
  // messageWidth, at least 170px, and never wider than the bubble.
  double imageWidth(double w, double h) =>
      (_messageWidth * w / h).clamp(170, _maxBubbleWidth).toDouble();

  for (final c in [
    (name: 'portrait 3:4', w: 3000.0, h: 4000.0),
    (name: 'portrait 9:16', w: 1080.0, h: 1920.0),
    (name: 'small portrait thumbnail', w: 120.0, h: 160.0),
    (name: 'landscape 4:3', w: 4000.0, h: 3000.0),
    (name: 'square', w: 1000.0, h: 1000.0),
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
      final expected = imageWidth(c.w, c.h);
      expect(bubbleWidth(tester), closeTo(expected, 0.5));
      expect(tester.getRect(find.byType(Image)).width, closeTo(expected, 0.5));
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
    expect(bubbleWidth(tester), closeTo(_maxBubbleWidth, 0.01));
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
    expect(bubbleWidth(tester), closeTo(_maxBubbleWidth, 0.5));
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
      expect(bubbleWidth(tester), closeTo(_maxBubbleWidth, 0.5));
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

  testWidgets('keeps photos within the media height cap on wide desktop panes', (
    tester,
  ) async {
    const desktopColumn = 1060.0;
    final contentWidth = (desktopColumn * ChatBubbleStyle.maxWidthFraction)
        .floor();
    expect(chatMediaExtent(contentWidth), kChatMediaMaxExtent);
    expect(chatMediaExtent(200), 200);

    for (final (w, h) in [(3000.0, 4000.0), (4000.0, 3000.0)]) {
      // ImageMessageWidget reads its size once, so start each case fresh.
      await tester.pumpWidget(const SizedBox.shrink());
      await pumpInBubble(
        tester,
        ImageMessageWidget(
          message: image(w, h),
          messageWidth: chatMediaExtent(contentWidth),
        ),
        column: desktopColumn,
      );
      final rect = tester.getRect(find.byType(Image));
      expect(rect.height, closeTo(kChatMediaMaxExtent, 0.5));
      expect(rect.width, closeTo(kChatMediaMaxExtent * w / h, 0.5));
    }
  });
}
