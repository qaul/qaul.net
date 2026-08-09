import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/chat_fixtures.dart';
import '../../../../support/widgetbook_preview.dart';

/// UI-only parent that connects message selection, reply composition, and the
/// resulting reply bubble.
///
/// Messages sent here are held locally. This component deliberately has no
/// backend, protocol, storage, or navigation knowledge.
class ChatReplyJourney extends StatefulWidget {
  const ChatReplyJourney.direct({
    super.key,
    required this.currentUser,
    required this.initialMessages,
    required this.placeholder,
    this.clock,
    this.currentUserReplyLabel,
    this.quickReactions = const [],
    this.onAddReaction,
    this.sendTooltip,
    this.cancelReplyTooltip,
    this.onMessageSent,
  }) : mode = ChatRenderMode.direct;

  const ChatReplyJourney.group({
    super.key,
    required this.currentUser,
    required this.initialMessages,
    required this.placeholder,
    this.clock,
    this.currentUserReplyLabel,
    this.quickReactions = const [],
    this.onAddReaction,
    this.sendTooltip,
    this.cancelReplyTooltip,
    this.onMessageSent,
  }) : mode = ChatRenderMode.group;

  final ChatUser currentUser;
  final List<ChatMessage> initialMessages;
  final String placeholder;
  final DateTime? clock;
  final String? currentUserReplyLabel;
  final List<ChatMessageQuickReaction> quickReactions;
  final VoidCallback? onAddReaction;
  final String? sendTooltip;
  final String? cancelReplyTooltip;
  final ValueChanged<TextChatMessage>? onMessageSent;
  final ChatRenderMode mode;

  @override
  State<ChatReplyJourney> createState() => _ChatReplyJourneyState();
}

class _ChatReplyJourneyState extends State<ChatReplyJourney> {
  late final TextEditingController _composerController;
  late final List<ChatMessage> _messages;
  final GlobalKey _timelineStackKey = GlobalKey();
  TextChatMessage? _contextMenuTarget;
  Offset? _contextMenuAnchor;
  TextChatMessage? _activeReplyTarget;
  var _localMessageSequence = 0;

  @override
  void initState() {
    super.initState();
    _composerController = TextEditingController();
    _messages = List<ChatMessage>.of(widget.initialMessages);
  }

  @override
  void dispose() {
    _composerController.dispose();
    super.dispose();
  }

  String _replyAuthor(TextChatMessage message) {
    if (message.sender.id == widget.currentUser.id) {
      return widget.currentUserReplyLabel ?? widget.currentUser.name;
    }
    return message.sender.name;
  }

  ChatFooterReplyPreviewData? get _footerReplyPreview {
    final target = _activeReplyTarget;
    if (target == null) return null;
    return ChatFooterReplyPreviewData(
      author: _replyAuthor(target),
      content: target.content,
    );
  }

  void _openContextMenu(TextChatMessage message, Offset globalPosition) {
    final renderBox =
        _timelineStackKey.currentContext?.findRenderObject() as RenderBox?;
    if (renderBox == null) return;
    setState(() {
      _contextMenuTarget = message;
      _contextMenuAnchor = renderBox.globalToLocal(globalPosition);
    });
  }

  void _closeContextMenu() {
    if (_contextMenuTarget == null) return;
    setState(() {
      _contextMenuTarget = null;
      _contextMenuAnchor = null;
    });
  }

  void _startReply() {
    final target = _contextMenuTarget;
    if (target == null) return;
    setState(() {
      _activeReplyTarget = target;
      _contextMenuTarget = null;
      _contextMenuAnchor = null;
    });
  }

  void _cancelReply() {
    if (_activeReplyTarget == null) return;
    setState(() => _activeReplyTarget = null);
  }

  void _send(String rawText) {
    final text = rawText.trim();
    if (text.isEmpty) return;

    final target = _activeReplyTarget;
    final sentAt = widget.clock ?? DateTime.now();
    final sentMessage = TextChatMessage(
      id: 'local-reply-${_localMessageSequence++}',
      sender: widget.currentUser,
      content: text,
      sentAt: sentAt,
      receivedAt: sentAt,
      status: MessageStatus.sent,
      replyPreview: target == null
          ? null
          : ChatReplyPreviewData(
              author: _replyAuthor(target),
              content: target.content,
            ),
    );

    setState(() {
      _messages.add(sentMessage);
      _activeReplyTarget = null;
      _contextMenuTarget = null;
      _contextMenuAnchor = null;
      _composerController.clear();
    });
    widget.onMessageSent?.call(sentMessage);
  }

  List<ChatMessageContextMenuElement> _contextMenuElements() {
    return [
      if (widget.quickReactions.isNotEmpty)
        ChatMessageReactionRow(
          reactions: widget.quickReactions,
          onAddReaction: widget.onAddReaction,
          showAddReaction: widget.onAddReaction != null,
        ),
      ChatMessageContextMenuAction.reply(onPressed: _startReply),
      const ChatMessageContextMenuAction.forward(enabled: false),
      const ChatMessageContextMenuAction.edit(enabled: false),
    ];
  }

  @override
  Widget build(BuildContext context) {
    final timeline = widget.mode == ChatRenderMode.group
        ? ChatTimeline.group(
            currentUser: widget.currentUser,
            messages: _messages,
            clock: widget.clock,
            onTextMessageLongPressStart: _openContextMenu,
            selectedTextMessageId: _contextMenuTarget?.id,
          )
        : ChatTimeline.direct(
            currentUser: widget.currentUser,
            messages: _messages,
            clock: widget.clock,
            onTextMessageLongPressStart: _openContextMenu,
            selectedTextMessageId: _contextMenuTarget?.id,
          );

    return ColoredBox(
      key: const ValueKey('chat-reply-flow'),
      color: QaulColorSheet(Theme.of(context).brightness).background,
      child: Column(
        children: [
          Expanded(
            child: Stack(
              key: _timelineStackKey,
              children: [
                Positioned.fill(
                  child: SingleChildScrollView(reverse: true, child: timeline),
                ),
                if (_contextMenuTarget != null &&
                    _contextMenuAnchor != null) ...[
                  Positioned.fill(
                    child: GestureDetector(
                      key: const ValueKey('chat-reply-menu-dismiss-area'),
                      behavior: HitTestBehavior.opaque,
                      onTap: _closeContextMenu,
                    ),
                  ),
                  Positioned.fill(
                    child: CustomSingleChildLayout(
                      delegate: _ContextMenuPositionDelegate(
                        anchor: _contextMenuAnchor!,
                      ),
                      child: ChatMessageContextMenu(
                        key: const ValueKey('chat-reply-context-menu'),
                        elements: _contextMenuElements(),
                      ),
                    ),
                  ),
                ],
              ],
            ),
          ),
          ChatFooter(
            placeholder: widget.placeholder,
            controller: _composerController,
            replyPreview: _footerReplyPreview,
            onCancelReply: _cancelReply,
            onSend: _send,
            sendTooltip: widget.sendTooltip,
            cancelReplyTooltip: widget.cancelReplyTooltip,
          ),
        ],
      ),
    );
  }
}

class _ContextMenuPositionDelegate extends SingleChildLayoutDelegate {
  const _ContextMenuPositionDelegate({required this.anchor});

  final Offset anchor;

  static const _margin = 12.0;
  static const _anchorGap = 20.0;

  @override
  BoxConstraints getConstraintsForChild(BoxConstraints constraints) {
    return constraints.loosen();
  }

  @override
  Offset getPositionForChild(Size size, Size childSize) {
    final maxLeft = (size.width - childSize.width - _margin).clamp(
      _margin,
      double.infinity,
    );
    final left = (anchor.dx - childSize.width / 2).clamp(_margin, maxLeft);

    final below = anchor.dy + _anchorGap;
    final above = anchor.dy - childSize.height - _anchorGap;
    final preferredTop = below + childSize.height <= size.height - _margin
        ? below
        : above;
    final maxTop = (size.height - childSize.height - _margin).clamp(
      _margin,
      double.infinity,
    );
    final top = preferredTop.clamp(_margin, maxTop);

    return Offset(left.toDouble(), top.toDouble());
  }

  @override
  bool shouldRelayout(_ContextMenuPositionDelegate oldDelegate) {
    return oldDelegate.anchor != anchor;
  }
}

final _replyJourneyClock = DateTime(2026, 4, 18, 12, 42);

const _replyQuickReactions = [
  ChatMessageQuickReaction(child: Text('❤️'), semanticLabel: 'Love'),
  ChatMessageQuickReaction(child: Text('👍'), semanticLabel: 'Like'),
  ChatMessageQuickReaction(child: Text('🔥'), semanticLabel: 'Fire'),
];

class _ReplyJourneyScreen extends StatelessWidget {
  const _ReplyJourneyScreen({required this.isGroup});

  final bool isGroup;

  @override
  Widget build(BuildContext context) {
    final messages = buildDirectChatFixtureMessages(clock: _replyJourneyClock);
    final journey = isGroup
        ? ChatReplyJourney.group(
            currentUser: chatFixtureCurrentUser,
            initialMessages: messages,
            placeholder: 'Group message',
            clock: _replyJourneyClock,
            currentUserReplyLabel: 'You',
            quickReactions: _replyQuickReactions,
            onAddReaction: () {},
            sendTooltip: 'Send',
            cancelReplyTooltip: 'Cancel reply',
          )
        : ChatReplyJourney.direct(
            currentUser: chatFixtureCurrentUser,
            initialMessages: messages,
            placeholder: 'Secure private message',
            clock: _replyJourneyClock,
            currentUserReplyLabel: 'You',
            quickReactions: _replyQuickReactions,
            onAddReaction: () {},
            sendTooltip: 'Send',
            cancelReplyTooltip: 'Cancel reply',
          );
    final header = isGroup
        ? ChatHeader.group(
            applyTopSafeArea: false,
            extraTopPadding: 24,
            onBackPressed: () {},
            avatar: chatFixtureAvatar(initials: 'G'),
            groupName: 'Group Name',
            membersCount: 12,
          )
        : ChatHeader(
            applyTopSafeArea: false,
            extraTopPadding: 24,
            onBackPressed: () {},
            avatar: chatFixtureAvatar(initials: 'M'),
            displayName: chatFixturePeer.name,
            isOnline: true,
            onlineLabel: 'online',
            lastSeenLabel: '',
          );

    return Material(
      color: widgetbookChatSurfaceColor(context),
      child: Column(
        children: [
          header,
          Expanded(child: journey),
        ],
      ),
    );
  }
}

@widgetbook.UseCase(
  name: 'Direct chat',
  type: ChatReplyJourney,
  path: '[design]/chat/reply_journey',
)
Widget buildDirectReplyJourneyUseCase(BuildContext context) {
  return const _ReplyJourneyScreen(isGroup: false);
}

@widgetbook.UseCase(
  name: 'Group chat',
  type: ChatReplyJourney,
  path: '[design]/chat/reply_journey',
)
Widget buildGroupReplyJourneyUseCase(BuildContext context) {
  return const _ReplyJourneyScreen(isGroup: true);
}
