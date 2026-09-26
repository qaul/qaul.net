import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/widgetbook_preview.dart';

/// Widgetbook's complete group-mention flow: open the picker, choose a
/// member, and send the resulting text-only message.
class ChatMentionJourneyDesignStory {
  const ChatMentionJourneyDesignStory();
}

const _currentUser = ChatUser(id: 'me', name: 'Me');
const _groupMember = ChatUser(id: 'group-member', name: 'Group Member');
const _groupMember2 = ChatUser(id: 'group-member-2', name: 'Groupmember 2');
const _thirdMember = ChatUser(id: 'third-member', name: 'Third Member');
final _clock = DateTime(2026, 4, 18, 21, 56);

const _mentionSuggestions = [
  ChatMentionSuggestion(id: 'group-member', label: 'Group Member'),
  ChatMentionSuggestion(id: 'group-member-2', label: 'Groupmember 2'),
  ChatMentionSuggestion(id: 'third-member', label: 'Third Member'),
  ChatMentionSuggestion(id: 'all', label: 'all', isEveryone: true),
];

List<ChatMessage> _initialMessages() => [
  TextChatMessage(
    id: 'mention-1',
    sender: _currentUser,
    content: 'Hello in 16px 300 font',
    sentAt: _clock.subtract(const Duration(hours: 5)),
    receivedAt: _clock.subtract(const Duration(hours: 5)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'mention-2',
    sender: _currentUser,
    content: 'This is a longer message with an addressing @Group Member',
    sentAt: _clock.subtract(const Duration(hours: 5, minutes: 1)),
    receivedAt: _clock.subtract(const Duration(hours: 5, minutes: 1)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'mention-3',
    sender: _groupMember,
    content: 'Writing @Third Member',
    sentAt: _clock.subtract(const Duration(hours: 3)),
    receivedAt: _clock.subtract(const Duration(hours: 3)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'mention-4',
    sender: _groupMember2,
    content: 'Another answer',
    sentAt: _clock.subtract(const Duration(hours: 2)),
    receivedAt: _clock.subtract(const Duration(hours: 2)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'mention-5',
    sender: _thirdMember,
    content: 'Longer message from the chatpartner @myname',
    sentAt: _clock.subtract(const Duration(minutes: 30)),
    receivedAt: _clock.subtract(const Duration(minutes: 30)),
    status: MessageStatus.sent,
  ),
];

class _ChatMentionJourney extends StatefulWidget {
  const _ChatMentionJourney();

  @override
  State<_ChatMentionJourney> createState() => _ChatMentionJourneyState();
}

class _ChatMentionJourneyState extends State<_ChatMentionJourney> {
  late final ChatMentionTextEditingController _controller;
  late final List<ChatMessage> _messages;
  var _nextMessage = 0;

  @override
  void initState() {
    super.initState();
    _controller = ChatMentionTextEditingController(
      text: 'Writing @',
      mentionLabels: _mentionSuggestions.map((item) => item.label),
    )..addListener(_onComposerChanged);
    _messages = _initialMessages();
  }

  @override
  void dispose() {
    _controller
      ..removeListener(_onComposerChanged)
      ..dispose();
    super.dispose();
  }

  void _onComposerChanged() => setState(() {});

  _MentionQuery? get _activeMention {
    final value = _controller.value;
    if (!value.selection.isValid || !value.selection.isCollapsed) return null;
    final cursor = value.selection.baseOffset;
    final text = value.text;
    if (cursor <= 0) return null;
    final index = text.lastIndexOf('@', cursor - 1);
    if (index == -1 ||
        (index > 0 && !RegExp(r'\s').hasMatch(text[index - 1])) ||
        text.substring(index + 1, cursor).contains(RegExp(r'\s'))) {
      return null;
    }
    return _MentionQuery(start: index, end: cursor);
  }

  List<ChatMentionSuggestion> get _visibleSuggestions {
    final mention = _activeMention;
    if (mention == null) return const [];
    final query = _controller.text
        .substring(mention.start + 1, mention.end)
        .toLowerCase();
    return _mentionSuggestions
        .where((item) => item.label.toLowerCase().contains(query))
        .toList(growable: false);
  }

  void _selectMention(ChatMentionSuggestion suggestion) {
    final mention = _activeMention;
    if (mention == null) return;
    final text = _controller.text;
    final replacement = '@${suggestion.label} ';
    _controller.value = TextEditingValue(
      text:
          '${text.substring(0, mention.start)}$replacement${text.substring(mention.end)}',
      selection: TextSelection.collapsed(
        offset: mention.start + replacement.length,
      ),
    );
  }

  void _send(String text) {
    final content = text.trim();
    if (content.isEmpty) return;
    setState(() {
      _messages.add(
        TextChatMessage(
          id: 'sent-mention-${_nextMessage++}',
          sender: _currentUser,
          content: content,
          sentAt: _clock,
          receivedAt: _clock,
          status: MessageStatus.sent,
        ),
      );
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: widgetbookChatSurfaceColor(context),
      child: ColoredBox(
        color: widgetbookChatCanvasColor(context),
        child: Column(
          children: [
            ChatHeader.group(
              applyTopSafeArea: false,
              extraTopPadding: 24,
              onBackPressed: () {},
              avatar: const QaulAvatar.group(size: QaulAvatarSize.small),
              groupName: 'Group Name',
              membersCount: 12,
              formatMembersCount: (count) => '$count Members',
              menuEntries: const [
                ChatHeaderMenuEntry(id: 'info', label: 'Info'),
              ],
              onMenuSelected: (_) {},
            ),
            Expanded(
              child: SingleChildScrollView(
                reverse: true,
                child: ChatTimeline.group(
                  currentUser: _currentUser,
                  messages: _messages,
                  clock: _clock,
                  mentionLabels: const [
                    'Group Member',
                    'Groupmember 2',
                    'Third Member',
                    'myname',
                  ],
                ),
              ),
            ),
            if (_visibleSuggestions.isNotEmpty)
              ChatMentionSuggestionList(
                suggestions: _visibleSuggestions,
                onSelected: _selectMention,
              ),
            ChatFooter(
              controller: _controller,
              placeholder: 'Secure private message',
              onSend: _send,
              onMoreAttachmentsPressed: () {},
              sendTooltip: 'Send',
            ),
          ],
        ),
      ),
    );
  }
}

class _MentionQuery {
  const _MentionQuery({required this.start, required this.end});

  final int start;
  final int end;
}

@widgetbook.UseCase(
  name: 'Group flow',
  type: ChatMentionJourneyDesignStory,
  path: '[design]/chat/mention_journey',
)
Widget buildGroupMentionJourneyUseCase(BuildContext context) =>
    const _ChatMentionJourney();
