import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  test('resolves mention backgrounds for both themes', () {
    expect(
      chatMentionComposerBackground(Brightness.dark),
      const Color(0xFF333333),
    );
    expect(
      chatMentionComposerBackground(Brightness.light),
      const Color(0xFFE5E5E5),
    );
    expect(
      chatMentionBubbleBackground(Brightness.dark),
      const Color(0x54020202),
    );
    expect(
      chatMentionBubbleBackground(Brightness.light),
      const Color(0x54FFFFFF),
    );
  });

  group('buildChatMentionTextSpan', () {
    const style = TextStyle(fontSize: 16, fontWeight: FontWeight.w300);

    test('emphasizes complete known mentions and preserves text', () {
      final span = buildChatMentionTextSpan(
        text: 'Writing @Third Member and @all.',
        style: style,
        mentionLabels: const ['Third Member'],
        mentionBackgroundColor: const Color(0xFF333333),
      );

      expect(span.toPlainText(), 'Writing @Third Member and @all.');
      final children = span.children!;
      expect(children, hasLength(5));
      expect(children[1].toPlainText(), '@Third Member');
      expect(children[1].style!.fontWeight, FontWeight.w700);
      expect(children[1].style!.backgroundColor, const Color(0xFF333333));
      expect(children[3].toPlainText(), '@all');
      expect(children[3].style!.fontWeight, FontWeight.w700);
    });

    test('does not emphasize a mention embedded in another word', () {
      final span = buildChatMentionTextSpan(
        text: 'email@Third Member and @Third Members',
        style: style,
        mentionLabels: const ['Third Member'],
      );

      expect(span.children, isNull);
      expect(span.toPlainText(), 'email@Third Member and @Third Members');
    });

    test('matches the longest label first', () {
      final span = buildChatMentionTextSpan(
        text: '@Anna K is here',
        style: style,
        mentionLabels: const ['Anna', 'Anna K'],
      );

      expect(span.children![0].toPlainText(), '@Anna K');
      expect(span.children![0].style!.fontWeight, FontWeight.w700);
    });
  });

  testWidgets('suggestion list calls back for its selected member', (
    tester,
  ) async {
    ChatMentionSuggestion? selection;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatMentionSuggestionList(
            suggestions: const [
              ChatMentionSuggestion(id: 'third', label: 'Third Member'),
            ],
            onSelected: (suggestion) => selection = suggestion,
          ),
        ),
      ),
    );

    await tester.tap(
      find.byKey(const ValueKey('chat-mention-suggestion-third')),
    );

    expect(selection?.label, 'Third Member');
  });
}
