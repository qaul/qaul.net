import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  Widget app(Widget child, {ThemeData? theme}) {
    return MaterialApp(
      theme: theme,
      home: Scaffold(body: child),
    );
  }

  ChatMessageContextMenuAction action(
    int index, {
    VoidCallback? onPressed,
    bool enabled = true,
    bool hidden = false,
  }) {
    return ChatMessageContextMenuAction(
      id: 'action-$index',
      label: 'Action $index',
      iconAsset: ChatMessageContextMenuIcons.info,
      onPressed: onPressed ?? () {},
      enabled: enabled,
      hidden: hidden,
    );
  }

  ChatMessageReactionRow reactions({
    bool enabled = true,
    VoidCallback? onReaction,
    VoidCallback? onAddReaction,
  }) {
    return ChatMessageReactionRow(
      enabled: enabled,
      reactions: [
        ChatMessageQuickReaction(
          child: const Text('❤️'),
          semanticLabel: 'Love',
          onPressed: onReaction ?? () {},
        ),
      ],
      onAddReaction: onAddReaction ?? () {},
    );
  }

  testWidgets('exposes reaction and dedicated action callbacks', (
    tester,
  ) async {
    var reactionCount = 0;
    var replyCount = 0;
    var forwardCount = 0;
    var editCount = 0;

    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            reactions(onReaction: () => reactionCount++),
            ChatMessageContextMenuAction.reply(onPressed: () => replyCount++),
            ChatMessageContextMenuAction.forward(
              onPressed: () => forwardCount++,
            ),
            ChatMessageContextMenuAction.edit(onEdit: () => editCount++),
          ],
        ),
      ),
    );

    expect(tester.getSize(find.byTooltip('Love')), const Size.square(40));
    expect(find.byTooltip('Next page'), findsNothing);
    await tester.tap(find.byTooltip('Love'));
    await tester.tap(find.text('Reply'));
    await tester.tap(find.text('Forward'));
    await tester.tap(find.text('Edit'));

    expect((reactionCount, replyCount, forwardCount, editCount), (1, 1, 1, 1));
  });

  testWidgets('paginates first and final pages into five slots', (
    tester,
  ) async {
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            reactions(),
            for (var index = 1; index <= 7; index++) action(index),
          ],
        ),
      ),
    );

    expect(find.byKey(const ValueKey('reaction-row')), findsOneWidget);
    expect(find.text('Action 1'), findsOneWidget);
    expect(find.text('Action 3'), findsOneWidget);
    expect(find.text('Action 4'), findsNothing);
    expect(find.byTooltip('Previous page'), findsNothing);
    expect(find.byTooltip('Next page'), findsOneWidget);

    await tester.tap(find.byTooltip('Next page'));
    await tester.pump();

    expect(find.byKey(const ValueKey('reaction-row')), findsNothing);
    expect(find.text('Action 3'), findsNothing);
    expect(find.text('Action 4'), findsOneWidget);
    expect(find.text('Action 7'), findsOneWidget);
    expect(find.byTooltip('Previous page'), findsOneWidget);
    expect(find.byTooltip('Next page'), findsNothing);
  });

  testWidgets('shows reaction row and four actions without pagination', (
    tester,
  ) async {
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            reactions(),
            for (var index = 1; index <= 4; index++) action(index),
          ],
        ),
      ),
    );

    expect(find.byKey(const ValueKey('reaction-row')), findsOneWidget);
    expect(find.text('Action 1'), findsOneWidget);
    expect(find.text('Action 4'), findsOneWidget);
    expect(find.byTooltip('Previous page'), findsNothing);
    expect(find.byTooltip('Next page'), findsNothing);
  });

  testWidgets('shows both navigation arrows on an intermediate page', (
    tester,
  ) async {
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            reactions(),
            for (var index = 1; index <= 9; index++) action(index),
          ],
        ),
      ),
    );

    await tester.tap(find.byTooltip('Next page'));
    await tester.pump();
    expect(find.byTooltip('Previous page'), findsOneWidget);
    expect(find.byTooltip('Next page'), findsOneWidget);
    expect(find.text('Action 4'), findsOneWidget);
    expect(find.text('Action 6'), findsOneWidget);

    await tester.tap(find.byTooltip('Next page'));
    await tester.pump();
    expect(find.byTooltip('Previous page'), findsOneWidget);
    expect(find.byTooltip('Next page'), findsNothing);
    expect(find.text('Action 7'), findsOneWidget);
    expect(find.text('Action 9'), findsOneWidget);
  });

  testWidgets('hidden elements do not occupy pagination slots', (tester) async {
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            action(1),
            action(2),
            action(3, hidden: true),
            action(4),
            action(5),
            action(6),
          ],
        ),
      ),
    );

    expect(find.text('Action 3'), findsNothing);
    expect(find.text('Action 6'), findsOneWidget);
    expect(find.byTooltip('Next page'), findsNothing);
  });

  testWidgets('disabled is dimmer than enabled and ignores hover and tap', (
    tester,
  ) async {
    var disabledTaps = 0;
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            action(1),
            action(2, enabled: false, onPressed: () => disabledTaps++),
          ],
        ),
        theme: ThemeData.dark(),
      ),
    );

    Color foreground(String label) {
      final button = tester.widget<TextButton>(
        find.ancestor(
          of: find.text(label),
          matching: find.bySubtype<TextButton>(),
        ),
      );
      return button.style!.foregroundColor!.resolve({})!;
    }

    expect(foreground('Action 1'), const Color(0xFF999999));
    expect(foreground('Action 2'), const Color(0xFF5F5F5F));

    final mouse = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await mouse.addPointer();
    await mouse.moveTo(tester.getCenter(find.text('Action 2')));
    await tester.pump();
    expect(foreground('Action 2'), const Color(0xFF5F5F5F));

    await tester.tap(find.text('Action 2'));
    expect(disabledTaps, 0);

    await mouse.moveTo(tester.getCenter(find.text('Action 1')));
    await tester.pump();
    expect(foreground('Action 1'), Colors.white);
  });

  testWidgets('disabled reaction row disables reactions and add button', (
    tester,
  ) async {
    var reactionTaps = 0;
    var addTaps = 0;
    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [
            reactions(
              enabled: false,
              onReaction: () => reactionTaps++,
              onAddReaction: () => addTaps++,
            ),
          ],
        ),
      ),
    );

    await tester.tap(find.byTooltip('Love'));
    await tester.tap(find.byTooltip('More reactions'));
    expect((reactionTaps, addTaps), (0, 0));
  });

  testWidgets('menu height shrinks when a page has fewer rows', (tester) async {
    await tester.pumpWidget(app(ChatMessageContextMenu(elements: [action(1)])));
    final shortHeight = tester
        .getSize(find.byType(ChatMessageContextMenu))
        .height;

    await tester.pumpWidget(
      app(
        ChatMessageContextMenu(
          elements: [for (var index = 1; index <= 5; index++) action(index)],
        ),
      ),
    );
    final fullHeight = tester
        .getSize(find.byType(ChatMessageContextMenu))
        .height;

    expect(shortHeight, lessThan(fullHeight));
  });

  testWidgets('renders the outlined menu window', (tester) async {
    await tester.pumpWidget(app(const ChatMessageContextMenu(elements: [])));

    final menuMaterials = find.descendant(
      of: find.byType(ChatMessageContextMenu),
      matching: find.byType(Material),
    );
    final material = tester.widget<Material>(menuMaterials.first);
    final shape = material.shape! as RoundedRectangleBorder;
    expect(shape.side.color, const Color(0xFF999999));
    expect(shape.side.width, 1);
  });
}
