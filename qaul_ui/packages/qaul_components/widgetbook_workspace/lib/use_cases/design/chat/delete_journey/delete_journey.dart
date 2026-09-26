import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/widgetbook_preview.dart';

/// UI-only delete-message journey.
///
/// It deliberately holds the confirmation and deleted state locally so the
/// design can be reviewed before the protocol and persistence work exist.
class ChatDeleteJourney extends StatefulWidget {
  const ChatDeleteJourney.direct({super.key}) : mode = ChatRenderMode.direct;

  const ChatDeleteJourney.group({super.key}) : mode = ChatRenderMode.group;

  final ChatRenderMode mode;

  @override
  State<ChatDeleteJourney> createState() => _ChatDeleteJourneyState();
}

const _currentUser = ChatUser(id: 'me', name: 'Me');
const _peer = ChatUser(id: 'maxx', name: 'MaxX');
const _groupMember = ChatUser(id: 'group-member', name: 'Group Member');
const _groupMember2 = ChatUser(id: 'group-member-2', name: 'Groupmember 2');
const _thirdMember = ChatUser(id: 'third-member', name: 'Third Member');
final _clock = DateTime(2026, 4, 18, 22, 30);

List<ChatMessage> _directMessages() => [
  TextChatMessage(
    id: 'direct-hello',
    sender: _currentUser,
    content: 'Hello in 16px 300 font',
    sentAt: _clock.subtract(const Duration(hours: 6, minutes: 20)),
    receivedAt: _clock.subtract(const Duration(hours: 6, minutes: 20)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'direct-delete',
    sender: _currentUser,
    content:
        'This is a longer message with no own timestamp followed by another message with timestamp',
    sentAt: _clock.subtract(const Duration(hours: 6, minutes: 10)),
    receivedAt: _clock.subtract(const Duration(hours: 6, minutes: 10)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'direct-answer-1',
    sender: _peer,
    content: 'Chatpartner is answering',
    sentAt: _clock.subtract(const Duration(hours: 4, minutes: 21)),
    receivedAt: _clock.subtract(const Duration(hours: 4, minutes: 21)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'direct-answer-2',
    sender: _peer,
    content: 'Another answer',
    sentAt: _clock.subtract(const Duration(hours: 4, minutes: 1)),
    receivedAt: _clock.subtract(const Duration(hours: 4, minutes: 1)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'direct-message',
    sender: _currentUser,
    content: 'Message',
    sentAt: _clock.subtract(const Duration(hours: 3, minutes: 7)),
    receivedAt: _clock.subtract(const Duration(hours: 3, minutes: 7)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'direct-long',
    sender: _peer,
    content: 'Longer message from the chatpartner',
    sentAt: _clock.subtract(const Duration(hours: 1, minutes: 11)),
    receivedAt: _clock.subtract(const Duration(hours: 1, minutes: 11)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'direct-latest',
    sender: _currentUser,
    content: 'New Message not out',
    sentAt: _clock,
    receivedAt: _clock,
    status: MessageStatus.notSent,
  ),
];

List<ChatMessage> _groupMessages() => [
  TextChatMessage(
    id: 'group-hello',
    sender: _currentUser,
    content: 'Hello in 16px 300 font',
    sentAt: _clock.subtract(const Duration(hours: 6, minutes: 20)),
    receivedAt: _clock.subtract(const Duration(hours: 6, minutes: 20)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'group-long-outgoing',
    sender: _currentUser,
    content:
        'This is a longer message with no own timestamp followed by another message with timestamp',
    sentAt: _clock.subtract(const Duration(hours: 6, minutes: 10)),
    receivedAt: _clock.subtract(const Duration(hours: 6, minutes: 10)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'group-answer',
    sender: _groupMember,
    content: 'Chatpartner is answering',
    sentAt: _clock.subtract(const Duration(hours: 4, minutes: 21)),
    receivedAt: _clock.subtract(const Duration(hours: 4, minutes: 21)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'group-delete',
    sender: _groupMember2,
    content: 'Another answer',
    sentAt: _clock.subtract(const Duration(hours: 4, minutes: 1)),
    receivedAt: _clock.subtract(const Duration(hours: 4, minutes: 1)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'group-message',
    sender: _currentUser,
    content: 'Message',
    sentAt: _clock.subtract(const Duration(hours: 3, minutes: 7)),
    receivedAt: _clock.subtract(const Duration(hours: 3, minutes: 7)),
    status: MessageStatus.read,
  ),
  TextChatMessage(
    id: 'group-long',
    sender: _thirdMember,
    content: 'Longer message from the chatpartner',
    sentAt: _clock.subtract(const Duration(hours: 1, minutes: 11)),
    receivedAt: _clock.subtract(const Duration(hours: 1, minutes: 11)),
    status: MessageStatus.sent,
  ),
  TextChatMessage(
    id: 'group-latest',
    sender: _thirdMember,
    content: 'Followed by one late night',
    sentAt: _clock,
    receivedAt: _clock,
    status: MessageStatus.sent,
  ),
];

class _ChatDeleteJourneyState extends State<ChatDeleteJourney> {
  final _timelineStackKey = GlobalKey();
  final _messageKeys = <String, GlobalKey>{};
  late final TextEditingController _composerController;
  late final List<ChatMessage> _messages;
  TextChatMessage? _contextMenuTarget;
  Rect? _contextMenuTargetRect;
  TextChatMessage? _confirmationTarget;
  Rect? _confirmationTargetRect;
  String? _deletedMessageId;

  bool get _isGroup => widget.mode == ChatRenderMode.group;

  @override
  void initState() {
    super.initState();
    _composerController = TextEditingController();
    _messages = _isGroup ? _groupMessages() : _directMessages();
  }

  @override
  void dispose() {
    _composerController.dispose();
    super.dispose();
  }

  void _openContextMenu(TextChatMessage message, Offset globalPosition) {
    if (message.id == _deletedMessageId) return;
    final timelineBox =
        _timelineStackKey.currentContext?.findRenderObject() as RenderBox?;
    final messageBox =
        _messageKeys[message.id]?.currentContext?.findRenderObject()
            as RenderBox?;
    if (timelineBox == null || messageBox == null) return;
    final topLeft = messageBox.localToGlobal(
      Offset.zero,
      ancestor: timelineBox,
    );
    final targetRect = topLeft & messageBox.size;
    setState(() {
      _contextMenuTarget = message;
      _contextMenuTargetRect = targetRect;
      _confirmationTarget = null;
      _confirmationTargetRect = null;
    });
  }

  void _dismissOverlays() {
    if (_contextMenuTarget == null && _confirmationTarget == null) return;
    setState(() {
      _contextMenuTarget = null;
      _contextMenuTargetRect = null;
      _confirmationTarget = null;
      _confirmationTargetRect = null;
    });
  }

  void _requestDelete() {
    final target = _contextMenuTarget;
    final targetRect = _contextMenuTargetRect;
    if (target == null || targetRect == null) return;
    setState(() {
      _contextMenuTarget = null;
      _contextMenuTargetRect = null;
      _confirmationTarget = target;
      _confirmationTargetRect = targetRect;
    });
  }

  void _confirmDelete() {
    final target = _confirmationTarget;
    if (target == null) return;
    setState(() {
      _deletedMessageId = target.id;
      _confirmationTarget = null;
      _confirmationTargetRect = null;
    });
  }

  List<ChatMessageContextMenuElement> _contextMenuElements() => [
    const ChatMessageContextMenuAction(
      id: 'info',
      label: 'Info',
      iconAsset: ChatMessageContextMenuIcons.info,
      enabled: false,
    ),
    const ChatMessageContextMenuAction(
      id: 'share',
      label: 'Share',
      iconAsset: ChatMessageContextMenuIcons.share,
      enabled: false,
    ),
    const ChatMessageContextMenuAction(
      id: 'copy',
      label: 'Copy',
      iconAsset: ChatMessageContextMenuIcons.copy,
      enabled: false,
    ),
    ChatMessageContextMenuAction(
      id: 'delete',
      label: 'Delete',
      iconAsset: ChatMessageContextMenuIcons.delete,
      onPressed: _requestDelete,
    ),
  ];

  @override
  Widget build(BuildContext context) {
    final header = _isGroup
        ? ChatHeader.group(
            applyTopSafeArea: false,
            extraTopPadding: 24,
            onBackPressed: () {},
            avatar: const QaulAvatar.group(size: QaulAvatarSize.small),
            groupName: 'Group Name',
            membersCount: 12,
            formatMembersCount: (count) => '$count Members',
            menuEntries: const [ChatHeaderMenuEntry(id: 'info', label: 'Info')],
            onMenuSelected: (_) {},
          )
        : ChatHeader(
            applyTopSafeArea: false,
            extraTopPadding: 24,
            onBackPressed: () {},
            avatar: chatAvatar(initials: 'M'),
            displayName: 'MaxX',
            isOnline: true,
            onlineLabel: 'online',
            lastSeenLabel: '',
          );

    return Material(
      color: widgetbookChatSurfaceColor(context),
      child: ColoredBox(
        color: widgetbookChatCanvasColor(context),
        child: Column(
          children: [
            header,
            Expanded(
              child: Stack(
                key: _timelineStackKey,
                children: [
                  Positioned.fill(
                    child: SingleChildScrollView(
                      reverse: true,
                      child: _DeleteJourneyTimeline(
                        mode: widget.mode,
                        currentUser: _currentUser,
                        messages: _messages,
                        clock: _clock,
                        deletedMessageId: _deletedMessageId,
                        selectedMessageId:
                            _contextMenuTarget?.id ?? _confirmationTarget?.id,
                        messageKeyFor: (message) =>
                            _messageKeys.putIfAbsent(message.id, GlobalKey.new),
                        onTextMessageLongPressStart: _openContextMenu,
                      ),
                    ),
                  ),
                  if (_contextMenuTarget != null &&
                      _contextMenuTargetRect != null) ...[
                    Positioned.fill(
                      child: GestureDetector(
                        behavior: HitTestBehavior.opaque,
                        onTap: _dismissOverlays,
                      ),
                    ),
                    Positioned.fill(
                      child: CustomSingleChildLayout(
                        delegate: _OverlayPositionDelegate(
                          targetRect: _contextMenuTargetRect!,
                          preferredWidth: ChatMessageContextMenu.width,
                        ),
                        child: ChatMessageContextMenu(
                          key: const ValueKey('chat-delete-context-menu'),
                          elements: _contextMenuElements(),
                        ),
                      ),
                    ),
                  ],
                  if (_confirmationTarget != null &&
                      _confirmationTargetRect != null) ...[
                    Positioned.fill(
                      child: GestureDetector(
                        behavior: HitTestBehavior.opaque,
                        onTap: _dismissOverlays,
                      ),
                    ),
                    Positioned.fill(
                      child: CustomSingleChildLayout(
                        delegate: _OverlayPositionDelegate(
                          targetRect: _confirmationTargetRect!,
                          preferredWidth: 200,
                          preferAdjacentToTarget: true,
                        ),
                        child: _DeleteConfirmationCard(
                          onClose: _dismissOverlays,
                          onDelete: _confirmDelete,
                        ),
                      ),
                    ),
                  ],
                ],
              ),
            ),
            ChatFooter(
              controller: _composerController,
              placeholder: _isGroup
                  ? 'Secure private message'
                  : 'Private message',
              onSend: (_) {},
              onMoreAttachmentsPressed: () {},
              sendTooltip: 'Send',
            ),
          ],
        ),
      ),
    );
  }
}

class _DeleteJourneyTimeline extends StatelessWidget {
  const _DeleteJourneyTimeline({
    required this.mode,
    required this.currentUser,
    required this.messages,
    required this.clock,
    required this.deletedMessageId,
    required this.selectedMessageId,
    required this.messageKeyFor,
    required this.onTextMessageLongPressStart,
  });

  final ChatRenderMode mode;
  final ChatUser currentUser;
  final List<ChatMessage> messages;
  final DateTime clock;
  final String? deletedMessageId;
  final String? selectedMessageId;
  final GlobalKey Function(TextChatMessage message) messageKeyFor;
  final void Function(TextChatMessage message, Offset globalPosition)
  onTextMessageLongPressStart;

  bool _isOutgoing(TextChatMessage message) =>
      message.sender.id == currentUser.id;

  QaulGroupMessageSender _groupSender(TextChatMessage message) =>
      QaulGroupMessageSender(
        idBase58: message.sender.id,
        name: message.sender.name,
      );

  @override
  Widget build(BuildContext context) {
    final textMessages = messages.whereType<TextChatMessage>().toList();
    final presentations = computeChatMessagePresentation(
      ascendingTimeline: [
        for (final message in textMessages)
          ChatTimelinePresentationRow(
            messageIdBase58: message.id,
            senderIdBase58: message.sender.id,
            sentAt: message.sentAt,
            isText: true,
            isOutgoing: _isOutgoing(message),
            qaulBubbleBaseWithoutLayout: QaulChatBubbleMessage(
              content: message.content,
              sentAt: message.sentAt,
              receivedAt: message.receivedAt,
              status: message.status,
              messageType: _isOutgoing(message)
                  ? MessageType.primary
                  : MessageType.secondary,
              edges: const [],
              senderIdBase58: message.sender.id,
              replyPreview: message.replyPreview,
            ),
          ),
      ],
      layoutMode: mode,
    );

    DateTime? lastDay;
    final children = <Widget>[];
    for (final message in textMessages) {
      final day = DateTime(
        message.sentAt.year,
        message.sentAt.month,
        message.sentAt.day,
      );
      if (lastDay == null || !_sameDay(lastDay, day)) {
        children.add(
          Padding(
            padding: EdgeInsets.only(
              top: lastDay == null ? 0 : kChatBubbleSeparatedGap,
              bottom: 16,
            ),
            child: RoomMetaMessage.date(date: day),
          ),
        );
        lastDay = day;
      }

      final computation = presentations[message.id]!;
      final sender = _isOutgoing(message) ? null : _groupSender(message);
      final presentation = MessagePresentation.fromComputation(
        messageId: message.id,
        sender: sender,
        computation: computation,
      );
      final isDeleted = message.id == deletedMessageId;
      final child = isDeleted
          ? _DeletedMessageItem(
              message: message,
              presentation: presentation,
              mode: mode,
              currentUser: currentUser,
              clock: clock,
            )
          : ChatMessageRenderer.renderText(
              presentation: presentation,
              mode: mode,
              clock: clock,
              isSelected: message.id == selectedMessageId,
            );
      children.add(
        GestureDetector(
          key: messageKeyFor(message),
          behavior: HitTestBehavior.opaque,
          onLongPressStart: isDeleted
              ? null
              : (details) => onTextMessageLongPressStart(
                  message,
                  details.globalPosition,
                ),
          child: child,
        ),
      );
    }

    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: children,
      ),
    );
  }
}

class _DeletedMessageItem extends StatelessWidget {
  const _DeletedMessageItem({
    required this.message,
    required this.presentation,
    required this.mode,
    required this.currentUser,
    required this.clock,
  });

  final TextChatMessage message;
  final MessagePresentation presentation;
  final ChatRenderMode mode;
  final ChatUser currentUser;
  final DateTime clock;

  @override
  Widget build(BuildContext context) {
    final isOutgoing = message.sender.id == currentUser.id;
    final marker = _DeletedMessageMarker(
      label: isOutgoing ? 'You deleted this message' : 'Message deleted',
      message: message,
      isOutgoing: isOutgoing,
      showTimestamp: presentation.meta.showTimestamp,
      senderName: presentation.meta.showSenderName
          ? presentation.sender?.name
          : null,
    );

    if (mode == ChatRenderMode.group && !isOutgoing) {
      return GroupMessageShell(
        marginTop: presentation.meta.topSpacing,
        sender: presentation.sender,
        showSenderName: false,
        showSenderAvatar: presentation.meta.showAvatar,
        child: marker,
      );
    }

    return Padding(
      padding: EdgeInsetsDirectional.only(
        top: presentation.meta.topSpacing,
        start: isOutgoing ? 0 : 16,
        end: isOutgoing ? 16 : 0,
      ),
      child: Align(
        alignment: isOutgoing
            ? AlignmentDirectional.centerEnd
            : AlignmentDirectional.centerStart,
        child: marker,
      ),
    );
  }
}

class _DeletedMessageMarker extends StatelessWidget {
  const _DeletedMessageMarker({
    required this.label,
    required this.message,
    required this.isOutgoing,
    required this.showTimestamp,
    required this.senderName,
  });

  final String label;
  final TextChatMessage message;
  final bool isOutgoing;
  final bool showTimestamp;
  final String? senderName;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final foreground = isDark ? Colors.white : const Color(0xFF252525);
    final timeMessage = QaulChatBubbleMessage(
      content: label,
      sentAt: message.sentAt,
      receivedAt: message.receivedAt,
      status: message.status,
      messageType: isOutgoing ? MessageType.primary : MessageType.secondary,
      edges: const [],
    );

    return ConstrainedBox(
      constraints: const BoxConstraints(
        maxWidth: ChatBubbleStyle.maxBubbleWidthMobile,
      ),
      child: DecoratedBox(
        decoration: ShapeDecoration(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(10),
            side: BorderSide(color: foreground.withValues(alpha: 0.8)),
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (senderName != null)
                Padding(
                  padding: const EdgeInsets.only(bottom: 3),
                  child: Text(
                    senderName!,
                    style: kGroupSenderNameTextStyle.copyWith(
                      color: foreground.withValues(alpha: 0.85),
                    ),
                  ),
                ),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Flexible(
                    child: Text(
                      label,
                      style: TextStyle(
                        color: foreground,
                        fontFamily: 'Roboto',
                        fontSize: 12,
                        fontWeight: FontWeight.w300,
                        height: 1.3,
                      ),
                    ),
                  ),
                  if (showTimestamp) ...[
                    const SizedBox(width: 4),
                    Text(
                      formatQaulChatBubbleTime(timeMessage, _clock),
                      style: TextStyle(
                        color: foreground.withValues(alpha: 0.75),
                        fontFamily: 'Roboto',
                        fontSize: 11,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                    if (isOutgoing && message.status != MessageStatus.notSent)
                      Padding(
                        padding: const EdgeInsetsDirectional.only(start: 3),
                        child: Icon(
                          message.status == MessageStatus.read
                              ? Icons.done_all
                              : Icons.check,
                          size: 14,
                          color: foreground.withValues(
                            alpha: message.status == MessageStatus.read
                                ? 0.9
                                : 0.8,
                          ),
                        ),
                      ),
                  ],
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _DeleteConfirmationCard extends StatelessWidget {
  const _DeleteConfirmationCard({
    required this.onClose,
    required this.onDelete,
  });

  final VoidCallback onClose;
  final VoidCallback onDelete;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final foreground = isDark ? Colors.white : const Color(0xFF252525);
    final surface = isDark ? const Color(0xFF282828) : const Color(0xFFF1F1F1);
    final destructiveActionColor = isDark
        ? const Color(0xFF999999)
        : const Color(0xFF666666);
    return Material(
      color: surface,
      elevation: 8,
      shadowColor: Colors.black.withValues(alpha: 0.35),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(15),
        side: const BorderSide(color: Color(0xFF999999), width: 0.75),
      ),
      child: SizedBox(
        width: 200,
        height: 166,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 6, 12, 16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Align(
                alignment: AlignmentDirectional.centerEnd,
                child: IconButton(
                  tooltip: 'Close',
                  onPressed: onClose,
                  padding: EdgeInsets.zero,
                  constraints: const BoxConstraints.tightFor(
                    width: 32,
                    height: 32,
                  ),
                  iconSize: 28,
                  icon: Icon(
                    Icons.close,
                    color: foreground.withValues(alpha: 0.6),
                  ),
                ),
              ),
              Text(
                'Note: A deleted message may remain visible to chat partners until the chat history is synchronized.',
                style: TextStyle(
                  color: foreground,
                  fontFamily: 'Roboto',
                  fontSize: 12,
                  fontWeight: FontWeight.w300,
                  height: 1.3,
                ),
              ),
              const SizedBox(height: 16),
              InkWell(
                onTap: onDelete,
                child: Text(
                  'Delete message',
                  style: TextStyle(
                    color: destructiveActionColor,
                    fontFamily: 'Roboto',
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                    height: 1.2,
                    decoration: TextDecoration.underline,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _OverlayPositionDelegate extends SingleChildLayoutDelegate {
  const _OverlayPositionDelegate({
    required this.targetRect,
    required this.preferredWidth,
    this.preferAdjacentToTarget = false,
  });

  final Rect targetRect;
  final double preferredWidth;
  final bool preferAdjacentToTarget;

  static const _margin = 12.0;
  static const _anchorGap = 16.0;

  @override
  BoxConstraints getConstraintsForChild(BoxConstraints constraints) {
    return constraints.loosen().enforce(
      BoxConstraints(maxWidth: preferredWidth),
    );
  }

  @override
  Offset getPositionForChild(Size size, Size childSize) {
    if (preferAdjacentToTarget) {
      final right = targetRect.right + _anchorGap;
      final left = targetRect.left - childSize.width - _anchorGap;
      final canFitAtRight = right + childSize.width <= size.width - _margin;
      final canFitAtLeft = left >= _margin;
      if (canFitAtRight || canFitAtLeft) {
        final top = (targetRect.center.dy - childSize.height / 2).clamp(
          _margin,
          size.height - childSize.height - _margin,
        );
        return Offset(
          (canFitAtRight ? right : left).toDouble(),
          top.toDouble(),
        );
      }
    }

    final maxLeft = (size.width - childSize.width - _margin).clamp(
      _margin,
      double.infinity,
    );
    final left = (targetRect.center.dx - childSize.width / 2).clamp(
      _margin,
      maxLeft,
    );
    final below = targetRect.bottom + _anchorGap;
    final above = targetRect.top - childSize.height - _anchorGap;
    final top = below + childSize.height <= size.height - _margin
        ? below
        : above.clamp(_margin, size.height - childSize.height - _margin);
    return Offset(left.toDouble(), top.toDouble());
  }

  @override
  bool shouldRelayout(_OverlayPositionDelegate oldDelegate) =>
      oldDelegate.targetRect != targetRect ||
      oldDelegate.preferredWidth != preferredWidth ||
      oldDelegate.preferAdjacentToTarget != preferAdjacentToTarget;
}

bool _sameDay(DateTime left, DateTime right) =>
    left.year == right.year &&
    left.month == right.month &&
    left.day == right.day;

Widget chatAvatar({required String initials}) => QaulAvatar(
  name: initials,
  id: 'delete-journey-avatar-$initials',
  size: QaulAvatarSize.small,
);

@widgetbook.UseCase(
  name: 'Direct chat',
  type: ChatDeleteJourney,
  path: '[design]/chat/delete_journey',
)
Widget buildDirectDeleteJourneyUseCase(BuildContext context) =>
    const ChatDeleteJourney.direct();

@widgetbook.UseCase(
  name: 'Group chat',
  type: ChatDeleteJourney,
  path: '[design]/chat/delete_journey',
)
Widget buildGroupDeleteJourneyUseCase(BuildContext context) =>
    const ChatDeleteJourney.group();
