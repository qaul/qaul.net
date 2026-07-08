part of 'chat.dart';

/// Single owner of the `types.SystemMessage.metadata` wire-format that bridges
/// the design-system meta models ([GroupJoinMetaChatMessage],
/// [DuplicateUsernameMetaChatMessage]) to the flutter_chat_ui renderer. Every
/// metadata key lives here and nowhere else; producers speak in typed models
/// and this is the only place they become an untyped map. When the app renders
/// through [ChatTimeline] directly, this bridge disappears entirely.
const _kMetaKind = 'kind';
const _kGroupJoinMetaKind = 'group_join';
const _kDuplicateUsernameMetaKind = 'duplicate_username_on_join';

Map<String, dynamic> _encodeMetaMessage(ChatMessage message) {
  switch (message) {
    case GroupJoinMetaChatMessage():
      return {
        _kMetaKind: _kGroupJoinMetaKind,
        'userName': message.userName,
        'joinedSuffix': message.joinedSuffix,
      };
    case DuplicateUsernameMetaChatMessage():
      return {
        _kMetaKind: _kDuplicateUsernameMetaKind,
        'prefix': message.prefix,
        'emphasizedName': message.emphasizedName,
        'actionLabel': message.actionLabel,
      };
    default:
      throw ArgumentError('not a system meta message: ${message.runtimeType}');
  }
}

/// The design-system meta model a [types.SystemMessage] was encoded from, or
/// null when the system message carries no meta payload (plain centered text).
ChatMessage? _decodeMetaMessage(types.SystemMessage message) {
  final metadata = message.metadata;
  switch (metadata?[_kMetaKind]) {
    case _kGroupJoinMetaKind:
      return GroupJoinMetaChatMessage(
        id: message.id,
        userName: metadata!['userName'] as String,
        joinedSuffix: metadata['joinedSuffix'] as String,
      );
    case _kDuplicateUsernameMetaKind:
      return DuplicateUsernameMetaChatMessage(
        id: message.id,
        prefix: metadata!['prefix'] as String,
        emphasizedName: metadata['emphasizedName'] as String,
        actionLabel: metadata['actionLabel'] as String,
      );
    default:
      return null;
  }
}

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

  // Only group rooms disambiguate colliding display names.
  final duplicateNotifications = renderMode == ChatRenderMode.group
      ? _detectDuplicateUsernamesForRoom(
          room: room,
          messages: domainMessages,
          resolveAuthor: resolveAuthor,
          l10n: l10n,
        )
      : const <DuplicateUsernameOnJoinNotification>[];
  final notificationByJoinId = {
    for (final n in duplicateNotifications) n.afterMessageIdBase58: n,
  };
  final disambiguatedNameByUserId = {
    for (final n in duplicateNotifications)
      n.joiningUserIdBase58: n.disambiguatedName,
  };

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
    }

    // Surface a rename notice immediately after a colliding join.
    final notification = notificationByJoinId[m.messageIdBase58];
    if (notification != null) {
      internalMessages.add(
        _duplicateUsernameSystemMessage(notification, l10n: l10n),
      );
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
        name: disambiguatedNameByUserId[author.idBase58] ?? author.name,
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

/// Maps the room's members and join events onto the pure domain snapshots and
/// returns the resulting duplicate-username notifications in timeline order.
List<DuplicateUsernameOnJoinNotification> _detectDuplicateUsernamesForRoom({
  required ChatRoom room,
  required List<Message> messages,
  required User Function(Message m, AppLocalizations l10n) resolveAuthor,
  required AppLocalizations l10n,
}) {
  final members = [
    for (final member in room.members)
      GroupMemberNameSnapshot(
        idBase58: member.idBase58,
        name: member.name,
        // Pending (not-yet-accepted) invites don't count as active members.
        countsTowardDuplicateCheck:
            member.invitationState != InvitationState.sent,
      ),
  ];

  final joins = <GroupJoinEventSnapshot>[];
  for (final m in messages) {
    final content = m.content;
    if (content is! GroupEventContent ||
        content.type != GroupEventContentType.joined) {
      continue;
    }
    final author = resolveAuthor(m, l10n);
    final roomUser = room.members.firstWhereOrNull(
      (member) => member.id.equals(author.id),
    );
    joins.add(
      GroupJoinEventSnapshot(
        messageIdBase58: m.messageIdBase58,
        userIdBase58: author.idBase58,
        userName: author.name,
        sentAt: m.sentAt,
        receivedAt: m.receivedAt,
        skipJoinedAnnouncement:
            roomUser?.invitationState == InvitationState.sent,
      ),
    );
  }

  return detectDuplicateUsernameOnJoin(joins: joins, members: members);
}

/// A synthetic timeline row rendering the rename notice after a colliding join.
types.SystemMessage _duplicateUsernameSystemMessage(
  DuplicateUsernameOnJoinNotification notification, {
  required AppLocalizations l10n,
}) {
  final meta = DuplicateUsernameMetaChatMessage(
    id: notification.syntheticMessageIdBase58,
    prefix: l10n.groupMemberRenamedOnJoin(notification.baseName),
    emphasizedName: notification.disambiguatedName,
    actionLabel: l10n.editGroupUserNames,
  );
  return types.SystemMessage(
    id: meta.id,
    createdAt: notification.receivedAt.millisecondsSinceEpoch,
    text: '',
    metadata: _encodeMetaMessage(meta),
  );
}
