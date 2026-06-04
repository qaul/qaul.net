part of 'chat.dart';

const _kDuplicateUsernameMetaKind = 'duplicate_username_on_join';

bool memberCountsTowardDuplicateCheck(ChatRoomUser member) =>
    member.invitationState != InvitationState.sent;

List<GroupMemberNameSnapshot> groupMemberSnapshotsFromRoom(ChatRoom room) {
  return [
    for (final member in room.members)
      GroupMemberNameSnapshot(
        idBase58: member.idBase58,
        name: member.name,
        countsTowardDuplicateCheck: memberCountsTowardDuplicateCheck(member),
      ),
  ];
}

List<GroupJoinEventSnapshot> groupJoinSnapshotsFromMessages({
  required Iterable<Message> messages,
  required ChatRoom room,
  required User Function(Message m, AppLocalizations l10n) resolveAuthor,
  required AppLocalizations l10n,
}) {
  final joins = <GroupJoinEventSnapshot>[];

  for (final message in messages) {
    final content = message.content;
    if (content is! GroupEventContent ||
        content.type != GroupEventContentType.joined) {
      continue;
    }

    final author = resolveAuthor(message, l10n);
    var skipJoinedAnnouncement = false;
    final roomUser = room.members.firstWhereOrNull(
      (member) => member.id.equals(author.id),
    );
    if (roomUser?.invitationState == InvitationState.sent) {
      skipJoinedAnnouncement = true;
    }

    joins.add(
      GroupJoinEventSnapshot(
        messageIdBase58: message.messageIdBase58,
        userIdBase58: author.idBase58,
        userName: author.name,
        receivedAt: message.receivedAt,
        skipJoinedAnnouncement: skipJoinedAnnouncement,
      ),
    );
  }

  return joins;
}

List<DuplicateUsernameOnJoinNotification>
duplicateUsernameNotificationsForRoom({
  required ChatRoom room,
  required Iterable<Message> messages,
  required User Function(Message m, AppLocalizations l10n) resolveAuthor,
  required AppLocalizations l10n,
}) {
  if (!room.isGroupChatRoom) return const [];

  return detectDuplicateUsernameOnJoin(
    joins: groupJoinSnapshotsFromMessages(
      messages: messages,
      room: room,
      resolveAuthor: resolveAuthor,
      l10n: l10n,
    ),
    members: groupMemberSnapshotsFromRoom(room),
  );
}

DuplicateUsernameOnJoinNotification? duplicateUsernameNotificationAfter({
  required String messageIdBase58,
  required List<DuplicateUsernameOnJoinNotification> notifications,
}) {
  for (final notification in notifications) {
    if (notification.afterMessageIdBase58 == messageIdBase58) {
      return notification;
    }
  }
  return null;
}

/// Maps joining member ids to their locally disambiguated display labels.
Map<String, String> disambiguatedGroupDisplayNames(
  Iterable<DuplicateUsernameOnJoinNotification> notifications,
) {
  return {
    for (final notification in notifications)
      notification.joiningUserIdBase58: notification.disambiguatedName,
  };
}

types.SystemMessage duplicateUsernameSystemMessage({
  required DuplicateUsernameOnJoinNotification notification,
  required AppLocalizations l10n,
}) {
  return types.SystemMessage(
    id: notification.syntheticMessageIdBase58,
    createdAt: notification.receivedAt.millisecondsSinceEpoch,
    text: '',
    metadata: {
      'kind': _kDuplicateUsernameMetaKind,
      'baseName': notification.baseName,
      'disambiguatedName': notification.disambiguatedName,
      'preamble': l10n.groupMemberRenamedOnJoinPreamble,
      'middle': l10n.groupMemberRenamedOnJoinMiddle,
      'actionLabel': l10n.editGroupUserNames,
    },
  );
}
