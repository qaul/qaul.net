import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  const reply = ChatFooterReplyPreviewData(
    author: 'Group member 2',
    content: 'Another answer',
  );

  Widget app({
    ChatFooterReplyPreviewData? replyPreview,
    VoidCallback? onCancelReply,
    TextEditingController? controller,
    ThemeData? theme,
  }) {
    return MaterialApp(
      theme: theme,
      home: Scaffold(
        body: Align(
          alignment: Alignment.bottomCenter,
          child: ChatFooter(
            placeholder: 'Secure private message',
            controller: controller,
            replyPreview: replyPreview,
            onCancelReply: onCancelReply,
            applyBottomSafeArea: false,
          ),
        ),
      ),
    );
  }

  testWidgets('renders the regular footer without reply data', (tester) async {
    await tester.pumpWidget(app());

    expect(
      find.byKey(const ValueKey('chat-footer-reply-preview')),
      findsNothing,
    );
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Secure private message'), findsOneWidget);
  });

  testWidgets('renders a themed divider above the entire footer', (
    tester,
  ) async {
    Future<BorderSide> topDivider(ThemeData theme) async {
      await tester.pumpWidget(app(theme: theme));
      await tester.pumpAndSettle();
      final surface = tester.widget<DecoratedBox>(
        find.byKey(const ValueKey('chat-footer-surface')),
      );
      final border = (surface.decoration as BoxDecoration).border! as Border;
      return border.top;
    }

    final darkTheme = ThemeData.dark();
    final lightTheme = ThemeData.light();

    final darkDivider = await topDivider(darkTheme);
    expect(darkDivider.width, 0.5);
    expect(darkDivider.color, Colors.white);

    final lightDivider = await topDivider(lightTheme);
    expect(lightDivider.width, 0.5);
    expect(lightDivider.color, const Color(0xFFD1D1D6));
  });

  testWidgets('renders reply target above an editable composer', (
    tester,
  ) async {
    final controller = TextEditingController();
    addTearDown(controller.dispose);

    await tester.pumpWidget(app(replyPreview: reply, controller: controller));

    expect(
      find.byKey(const ValueKey('chat-footer-reply-preview')),
      findsOneWidget,
    );
    expect(find.text('Group member 2'), findsOneWidget);
    expect(find.text('Another answer'), findsOneWidget);

    final previewTop = tester
        .getTopLeft(find.byKey(const ValueKey('chat-footer-reply-preview')))
        .dy;
    final fieldTop = tester.getTopLeft(find.byType(TextField)).dy;
    expect(previewTop, lessThan(fieldTop));

    await tester.enterText(find.byType(TextField), 'Example for a reply');
    expect(controller.text, 'Example for a reply');
  });

  testWidgets('cancel button triggers onCancelReply', (tester) async {
    var cancelCount = 0;

    await tester.pumpWidget(
      app(replyPreview: reply, onCancelReply: () => cancelCount++),
    );

    await tester.tap(find.byKey(const ValueKey('cancel-chat-reply')));
    expect(cancelCount, 1);
  });

  testWidgets('long author and excerpt remain constrained', (tester) async {
    const longAuthor =
        'A participant with a display name that is much too long for the row';
    const longContent =
        'This is a long selected message that should be truncated after two '
        'lines so the reply preview cannot expand the composer indefinitely.';

    await tester.pumpWidget(
      app(
        replyPreview: const ChatFooterReplyPreviewData(
          author: longAuthor,
          content: longContent,
        ),
      ),
    );

    final authorText = tester.widget<Text>(find.text(longAuthor));
    final contentText = tester.widget<Text>(find.text(longContent));
    expect(authorText.maxLines, 1);
    expect(authorText.overflow, TextOverflow.ellipsis);
    expect(contentText.maxLines, 2);
    expect(contentText.overflow, TextOverflow.ellipsis);
  });

  testWidgets('uses theme-appropriate preview surfaces', (tester) async {
    Future<Color?> previewColor(ThemeData theme) async {
      await tester.pumpWidget(app(replyPreview: reply, theme: theme));
      await tester.pumpAndSettle();
      final preview = tester.widget<DecoratedBox>(
        find.byKey(const ValueKey('chat-footer-reply-preview')),
      );
      return (preview.decoration as BoxDecoration).color;
    }

    expect(await previewColor(ThemeData.dark()), const Color(0xFF2C2C2E));
    expect(await previewColor(ThemeData.light()), const Color(0xFFE5E5EA));
  });
}
