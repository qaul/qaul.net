import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  Widget app(Widget child) => MaterialApp(home: Scaffold(body: child));

  testWidgets('exposes action and reaction callbacks', (tester) async {
    var reply = 0;
    var forward = 0;
    var edit = 0;
    var dismiss = 0;
    var reaction = 0;

    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          quickReactions: [
            ChatMessageQuickReaction(
              child: const Text('❤️'),
              semanticLabel: 'Love',
              onPressed: () => reaction++,
            ),
          ],
          showAddReaction: false,
          onReply: () => reply++,
          onForward: () => forward++,
          onEdit: () => edit++,
          onDismiss: () => dismiss++,
        ),
      ),
    );

    expect(tester.getSize(find.byTooltip('Love')), const Size.square(40));
    await tester.tap(find.byTooltip('Love'));
    await tester.tap(find.text('Reply'));
    await tester.tap(find.text('Forward'));
    await tester.tap(find.text('Edit'));
    await tester.tap(find.byTooltip('Close menu'));

    expect((reaction, reply, forward, edit, dismiss), (1, 1, 1, 1, 1));
  });

  testWidgets('supports hidden and disabled actions', (tester) async {
    var edit = 0;
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          showAddReaction: false,
          showReply: false,
          forwardEnabled: false,
          onForward: () {},
          editEnabled: false,
          onEdit: () => edit++,
          showDismiss: false,
        ),
      ),
    );

    expect(find.text('Reply'), findsNothing);
    expect(find.text('Forward'), findsOneWidget);
    expect(find.text('Edit'), findsOneWidget);
    await tester.tap(find.text('Edit'));
    expect(edit, 0);
  });

  testWidgets('renders the outlined menu window', (tester) async {
    await tester.pumpWidget(
      app(const ChatMessageContextMenu(showDismiss: false)),
    );

    final menuMaterials = find.descendant(
      of: find.byType(ChatMessageContextMenu),
      matching: find.byType(Material),
    );
    final material = tester.widget<Material>(menuMaterials.first);
    final shape = material.shape! as RoundedRectangleBorder;
    expect(shape.side.color, const Color(0xFF999999));
    expect(shape.side.width, 1);
  });

  testWidgets('action color becomes active only while hovered', (tester) async {
    await tester.pumpWidget(
      app(ChatMessageContextMenu(showDismiss: false, onReply: () {})),
    );

    TextButton replyButton() => tester.widget<TextButton>(
      find.ancestor(
        of: find.text('Reply'),
        matching: find.bySubtype<TextButton>(),
      ),
    );

    Color foreground() => replyButton().style!.foregroundColor!.resolve({})!;

    expect(foreground(), const Color(0xFF999999));

    final mouse = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await mouse.addPointer();
    await mouse.moveTo(tester.getCenter(find.text('Reply')));
    await tester.pump();
    expect(foreground(), const Color(0xFF252525));

    await mouse.moveTo(Offset.zero);
    await tester.pump();
    expect(foreground(), const Color(0xFF999999));
  });

  testWidgets('reaction slots fit within the menu at large text scale', (
    tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: MediaQuery(
          data: MediaQueryData(textScaler: TextScaler.linear(1.3)),
          child: Scaffold(
            body: ChatMessageContextMenu(
              quickReactions: [
                ChatMessageQuickReaction(
                  child: Text('❤️', style: TextStyle(fontSize: 27)),
                  semanticLabel: 'Love',
                ),
                ChatMessageQuickReaction(
                  child: Text('👍', style: TextStyle(fontSize: 27)),
                  semanticLabel: 'Like',
                ),
                ChatMessageQuickReaction(
                  child: Text('🔥', style: TextStyle(fontSize: 27)),
                  semanticLabel: 'Fire',
                ),
              ],
            ),
          ),
        ),
      ),
    );

    expect(tester.takeException(), isNull);
  });
}
