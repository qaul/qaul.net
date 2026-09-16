import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

const _allSuggestions = [
  ChatMentionSuggestion(id: 'group-member', label: 'Group Member'),
  ChatMentionSuggestion(id: 'group-member-2', label: 'Groupmember 2'),
  ChatMentionSuggestion(id: 'third-member', label: 'Third Member'),
  ChatMentionSuggestion(id: 'all', label: 'all', isEveryone: true),
];

Widget _mentionFrame(BuildContext context, Widget child) {
  return Material(
    child: ColoredBox(
      color: Colors.black,
      child: Column(
        children: [
          const Expanded(child: SizedBox.expand()),
          child,
        ],
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Group members',
  type: ChatMentionSuggestionList,
  path: 'design_components/chat/mentions',
)
Widget buildChatMentionSuggestionsUseCase(BuildContext context) {
  return _mentionFrame(
    context,
    ChatMentionSuggestionList(suggestions: _allSuggestions, onSelected: (_) {}),
  );
}

@widgetbook.UseCase(
  name: 'Filtered members',
  type: ChatMentionSuggestionList,
  path: 'design_components/chat/mentions',
)
Widget buildFilteredChatMentionSuggestionsUseCase(BuildContext context) {
  return _mentionFrame(
    context,
    ChatMentionSuggestionList(
      suggestions: const [
        ChatMentionSuggestion(id: 'third-member', label: 'Third Member'),
      ],
      onSelected: (_) {},
    ),
  );
}

@widgetbook.UseCase(
  name: 'Composer — selected mention',
  type: ChatMentionSuggestionList,
  path: 'design_components/chat/mentions',
)
Widget buildSelectedChatMentionUseCase(BuildContext context) {
  return _mentionFrame(
    context,
    ChatFooter(
      controller: ChatMentionTextEditingController(
        text: 'Writing @Group Member',
        mentionLabels: const ['Group Member'],
      ),
      placeholder: 'Secure private message',
      onSend: (_) {},
      onMoreAttachmentsPressed: () {},
      sendTooltip: 'Send',
    ),
  );
}
