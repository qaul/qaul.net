part of 'chat.dart';

class ChatTimelineProjection {
  const ChatTimelineProjection({
    required this.internalMessages,
    required this.presentations,
  });

  final List<types.Message> internalMessages;
  final Map<String, MessagePresentation> presentations;
}

ChatTimelineProjection? buildChatTimelineProjection({
  required ChatRoom room,
  required User signedInUser,
  required AppLocalizations l10n,
  required ChatRenderMode renderMode,
  required WidgetRef ref,
  required User Function(Message m, AppLocalizations l10n) resolveAuthor,
}) {
  final domainMessages = room.messages?.sorted();
  if (domainMessages == null) return null;
  final chronologicalMessages = domainMessages.reversed.toList(growable: false);

  final duplicateUsernameNotifications = renderMode == ChatRenderMode.group
      ? duplicateUsernameNotificationsForRoom(
          room: room,
          messages: domainMessages,
          resolveAuthor: resolveAuthor,
          l10n: l10n,
        )
      : const <DuplicateUsernameOnJoinNotification>[];

  final internalMessages = <types.Message>[];
  final textMessages = <Message>[];

  for (final m in domainMessages) {
    final author = resolveAuthor(m, l10n);
    final internal = m.toInternalMessage(author, ref, l10n: l10n, room: room);

    if (m.content is TextMessageContent && internal is types.TextMessage) {
      internalMessages.add(internal.copyWith(status: null, showStatus: false));
      textMessages.add(m);
    } else {
      internalMessages.add(internal);
      final duplicateNotification = duplicateUsernameNotificationAfter(
        messageIdBase58: m.messageIdBase58,
        notifications: duplicateUsernameNotifications,
      );
      if (duplicateNotification != null) {
        internalMessages.add(
          duplicateUsernameSystemMessage(
            notification: duplicateNotification,
            l10n: l10n,
          ),
        );
      }
    }
  }

  final bubbleMessages = <QaulChatBubbleMessage>[];
  final bubbleIds = <String>[];

  for (final m in textMessages) {
    final isMe = m.senderId.equals(signedInUser.id);
    bubbleMessages.add(
      QaulChatBubbleMessage(
        content: (m.content as TextMessageContent).content,
        sentAt: m.sentAt,
        receivedAt: m.receivedAt,
        status: m.status == MessageState.sent
            ? MessageStatus.sent
            : (m.status == MessageState.confirmed ||
                      m.status == MessageState.confirmedByAll
                  ? MessageStatus.read
                  : MessageStatus.notSent),
        messageType: isMe ? MessageType.primary : MessageType.secondary,
        edges: const [],
        senderIdBase58: m.senderIdBase58,
      ),
    );
    bubbleIds.add(m.messageIdBase58);
  }

  final bubbleBaseById = <String, QaulChatBubbleMessage>{};
  for (var i = 0; i < bubbleIds.length; i++) {
    bubbleBaseById[bubbleIds[i]] = bubbleMessages[i];
  }

  final ascendingRows = <ChatTimelinePresentationRow>[
    for (final m in chronologicalMessages)
      ChatTimelinePresentationRow(
        messageIdBase58: m.messageIdBase58,
        senderIdBase58: m.senderIdBase58,
        sentAt: m.sentAt,
        isText: m.content is TextMessageContent,
        isOutgoing: m.senderId.equals(signedInUser.id),
        qaulBubbleBaseWithoutLayout: bubbleBaseById[m.messageIdBase58]
            ?.copyWith(edges: const []),
      ),
  ];

  final computed = computeChatMessagePresentation(
    ascendingTimeline: ascendingRows,
    layoutMode: renderMode,
  );

  final presentations = <String, MessagePresentation>{};
  for (final m in domainMessages) {
    final slice = computed[m.messageIdBase58]!;
    final incomingGroup =
        renderMode == ChatRenderMode.group && !slice.isPrimary;

    QaulGroupMessageSender? groupSender;
    if (incomingGroup) {
      final author = resolveAuthor(m, l10n);
      groupSender = QaulGroupMessageSender(
        idBase58: author.idBase58,
        name: author.name,
        isConnected: author.isConnected,
      );
    }

    presentations[m.messageIdBase58] = MessagePresentation.fromComputation(
      messageId: m.messageIdBase58,
      sender: groupSender,
      computation: slice,
    );
  }

  return ChatTimelineProjection(
    internalMessages: internalMessages,
    presentations: presentations,
  );
}
