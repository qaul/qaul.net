import 'package:flutter/material.dart';

import '../../styles/qaul_color_sheet.dart';
import '../users/qaul_avatar.dart';

Color chatMentionComposerBackground(Brightness brightness) =>
    QaulColorSheet(brightness).surfaceContainer;

Color chatMentionBubbleBackground(Brightness brightness) =>
    brightness == Brightness.dark
    ? const Color(0x54020202)
    : const Color(0x54FFFFFF);

/// A person (or the group-wide [isEveryone] target) available to be mentioned.
@immutable
class ChatMentionSuggestion {
  const ChatMentionSuggestion({
    required this.id,
    required this.label,
    this.isEveryone = false,
  });

  /// A stable identifier used for keys and avatar colour generation.
  final String id;

  /// The text inserted after the `@` sign.
  final String label;

  /// Whether this represents the group-wide `@all` mention.
  final bool isEveryone;
}

/// A composer controller that renders complete, known mentions in bold while
/// retaining the normal [TextEditingController] editing API.
class ChatMentionTextEditingController extends TextEditingController {
  ChatMentionTextEditingController({
    super.text,
    Iterable<String> mentionLabels = const [],
    this.mentionBackgroundColor,
  }) : _mentionLabels = List.unmodifiable(mentionLabels);

  List<String> _mentionLabels;
  final Color? mentionBackgroundColor;

  set mentionLabels(Iterable<String> labels) {
    _mentionLabels = List.unmodifiable(labels);
    notifyListeners();
  }

  @override
  TextSpan buildTextSpan({
    required BuildContext context,
    TextStyle? style,
    required bool withComposing,
  }) {
    return buildChatMentionTextSpan(
      text: text,
      style: style ?? const TextStyle(),
      mentionLabels: _mentionLabels,
      mentionBackgroundColor:
          mentionBackgroundColor ??
          chatMentionComposerBackground(Theme.of(context).brightness),
    );
  }
}

/// The selectable group-member list shown above the chat composer while a
/// mention is being composed.
///
/// The host owns filtering and insertion, so this stays reusable for any
/// composer implementation.
class ChatMentionSuggestionList extends StatelessWidget {
  const ChatMentionSuggestionList({
    super.key,
    required this.suggestions,
    required this.onSelected,
  });

  final List<ChatMentionSuggestion> suggestions;
  final ValueChanged<ChatMentionSuggestion> onSelected;

  @override
  Widget build(BuildContext context) {
    final sheet = QaulColorSheet(Theme.of(context).brightness);
    final isDark = Theme.of(context).brightness == Brightness.dark;
    return DecoratedBox(
      decoration: BoxDecoration(
        color: sheet.background,
        border: Border(
          top: BorderSide(color: sheet.chatFooterDivider, width: 1),
          bottom: BorderSide(color: sheet.chatFooterDivider, width: 1),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final suggestion in suggestions)
              _ChatMentionSuggestionTile(
                suggestion: suggestion,
                onSelected: onSelected,
                textColor: isDark ? Colors.white : const Color(0xFF252525),
              ),
          ],
        ),
      ),
    );
  }
}

class _ChatMentionSuggestionTile extends StatelessWidget {
  const _ChatMentionSuggestionTile({
    required this.suggestion,
    required this.onSelected,
    required this.textColor,
  });

  final ChatMentionSuggestion suggestion;
  final ValueChanged<ChatMentionSuggestion> onSelected;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final hoverColor = isDark
        ? Colors.white.withValues(alpha: 0.10)
        : Colors.black.withValues(alpha: 0.06);
    final avatar = suggestion.isEveryone
        ? const QaulAvatar.group(size: QaulAvatarSize.small)
        : QaulAvatar(
            name: suggestion.label,
            id: suggestion.id,
            size: QaulAvatarSize.small,
          );
    return Semantics(
      button: true,
      label: 'Mention ${suggestion.label}',
      child: Material(
        color: Colors.transparent,
        borderRadius: BorderRadius.circular(4),
        child: InkWell(
          key: ValueKey('chat-mention-suggestion-${suggestion.id}'),
          borderRadius: BorderRadius.circular(4),
          hoverColor: hoverColor,
          splashColor: hoverColor,
          onTap: () => onSelected(suggestion),
          child: SizedBox(
            height: 56,
            child: Padding(
              padding: const EdgeInsetsDirectional.only(start: 16),
              child: Row(
                children: [
                  avatar,
                  const SizedBox(width: 16),
                  Expanded(
                    child: Text(
                      suggestion.label,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        color: textColor,
                        fontFamily: 'Roboto',
                        fontSize: 17,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

/// Builds the rich text used by message bubbles, emphasizing only complete,
/// known mentions. Candidate labels are checked longest-first so names with a
/// common prefix (for example, "Ann" and "Anna K") remain unambiguous.
TextSpan buildChatMentionTextSpan({
  required String text,
  required TextStyle style,
  Iterable<String> mentionLabels = const [],
  Color? mentionBackgroundColor,
}) {
  final labels = <String>{'all'}
    ..addAll(mentionLabels.where((label) => label.trim().isNotEmpty));
  final sortedLabels = labels.toList()
    ..sort((left, right) => right.length.compareTo(left.length));

  if (sortedLabels.isEmpty || !text.contains('@')) {
    return TextSpan(text: text, style: style);
  }

  final spans = <TextSpan>[];
  var plainTextStart = 0;
  var cursor = 0;
  while (cursor < text.length) {
    final atIndex = text.indexOf('@', cursor);
    if (atIndex == -1) break;

    final hasValidPrefix = atIndex == 0 || !_isMentionWord(text[atIndex - 1]);
    final label = hasValidPrefix
        ? sortedLabels.firstWhere((candidate) {
            final end = atIndex + 1 + candidate.length;
            return text.startsWith(candidate, atIndex + 1) &&
                (end == text.length || !_isMentionWord(text[end]));
          }, orElse: () => '')
        : '';
    if (label.isEmpty) {
      cursor = atIndex + 1;
      continue;
    }

    if (plainTextStart < atIndex) {
      spans.add(TextSpan(text: text.substring(plainTextStart, atIndex)));
    }
    final end = atIndex + label.length + 1;
    spans.add(
      TextSpan(
        text: text.substring(atIndex, end),
        style: style.copyWith(
          fontWeight: FontWeight.w700,
          backgroundColor: mentionBackgroundColor,
        ),
      ),
    );
    plainTextStart = end;
    cursor = end;
  }

  if (spans.isEmpty) return TextSpan(text: text, style: style);
  if (plainTextStart < text.length) {
    spans.add(TextSpan(text: text.substring(plainTextStart)));
  }
  return TextSpan(style: style, children: spans);
}

bool _isMentionWord(String value) => RegExp(r'[A-Za-z0-9_]').hasMatch(value);
