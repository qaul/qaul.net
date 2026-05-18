import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/design_components/chat/chat_timeline.dart';
import 'package:qaul_components/models/chat_message.dart' as model;
import 'package:qaul_components/models/chat_user.dart';
import 'package:qaul_components/qaul_components.dart' show MessageStatus;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

const _alice = ChatUser(id: 'alice-id', name: 'Alice');
const _bob = ChatUser(id: 'bob-id', name: 'Bob');

final _day1 = DateTime(2026, 1, 10, 12, 0, 0);
final _day2 = DateTime(2026, 1, 11, 12, 0, 0);

model.TextChatMessage _text({
  required String id,
  required ChatUser sender,
  required String content,
  DateTime? sentAt,
}) {
  final at = sentAt ?? _day1;
  return model.TextChatMessage(
    id: id,
    sender: sender,
    content: content,
    sentAt: at,
    receivedAt: at,
    status: MessageStatus.read,
  );
}

Widget _wrap(Widget child) =>
    MaterialApp(home: Scaffold(body: SingleChildScrollView(child: child)));

/// Finds a [RichText] whose [TextSpan.text] equals [text].
/// Bubble content is rendered as [RichText], not [Text].
Finder _findBubbleText(String text) => find.byWidgetPredicate(
      (w) =>
          w is RichText &&
          w.text is TextSpan &&
          (w.text as TextSpan).text == text,
    );

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

void main() {
  group('ChatTimeline.direct', () {
    testWidgets('renders text bubbles for both sides', (tester) async {
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'Hello Bob!'),
        _text(id: 'm2', sender: _bob, content: 'Hey Alice!'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.direct(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      expect(_findBubbleText('Hello Bob!'), findsOneWidget);
      expect(_findBubbleText('Hey Alice!'), findsOneWidget);
    });

    testWidgets('inserts a date divider when messages span two days',
        (tester) async {
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'Day one'),
        _text(
          id: 'm2',
          sender: _bob,
          content: 'Day two',
          sentAt: _day2,
        ),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.direct(
            currentUser: _alice,
            messages: messages,
            clock: _day2.add(const Duration(hours: 1)),
          ),
        ),
      );

      // Two date dividers: one before day1 messages, one before day2 messages.
      // Jan 10 2026 = Saturday, Jan 11 2026 = Sunday.
      expect(find.text('Saturday, January 10, 2026'), findsOneWidget);
      expect(find.text('Sunday, January 11, 2026'), findsOneWidget);
    });

    testWidgets('outgoing detection: currentUser message aligns to the right',
        (tester) async {
      // Alice is currentUser — her message should use MessageType.primary
      // (rendered in an Align with Alignment.centerRight).
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'I sent this'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.direct(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      // The bubble is wrapped in an Align with Alignment.centerRight for primary.
      final align = tester.widget<Align>(
        find
            .ancestor(
              of: _findBubbleText('I sent this'),
              matching: find.byType(Align),
            )
            .last,
      );
      expect(align.alignment, equals(Alignment.centerRight));
    });
  });

  group('ChatTimeline.group', () {
    testWidgets('renders sender name above incoming text bubbles',
        (tester) async {
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _bob, content: 'Hi group'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.group(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      // Sender name is shown as a Text widget inside the bubble in group mode.
      expect(find.text('Bob'), findsOneWidget);
      expect(_findBubbleText('Hi group'), findsOneWidget);
    });

    testWidgets('MetaChatMessage entries render as labeled meta rows',
        (tester) async {
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'First message'),
        const model.MetaChatMessage(
          id: 'meta1',
          label: 'Bob joined the group',
        ),
        _text(id: 'm2', sender: _bob, content: 'Hello!'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.group(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      expect(find.text('Bob joined the group'), findsOneWidget);
      expect(_findBubbleText('First message'), findsOneWidget);
      expect(_findBubbleText('Hello!'), findsOneWidget);
    });
  });

  group('ChatTimeline — date divider logic', () {
    testWidgets(
        'a single date divider appears before the first TextChatMessage',
        (tester) async {
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'Only message'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.direct(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      // Exactly one date divider label for _day1.
      expect(find.text('Saturday, January 10, 2026'), findsOneWidget);
    });

    testWidgets(
        'MetaChatMessage between same-day text messages produces only one date divider',
        (tester) async {
      // All text messages on the same day, with a meta in between.
      // Expect: only one date divider.
      final messages = <model.ChatMessage>[
        _text(id: 'm1', sender: _alice, content: 'Hello'),
        const model.MetaChatMessage(id: 'meta1', label: 'Some event'),
        _text(id: 'm2', sender: _bob, content: 'World'),
      ];

      await tester.pumpWidget(
        _wrap(
          ChatTimeline.direct(
            currentUser: _alice,
            messages: messages,
            clock: _day1.add(const Duration(hours: 1)),
          ),
        ),
      );

      expect(find.text('Saturday, January 10, 2026'), findsOneWidget);
      expect(find.text('Some event'), findsOneWidget);
    });
  });
}
