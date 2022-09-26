part of 'chat_room.dart';

enum ChatRoomUserRole { normal, admin, unknown }

enum InvitationState { sent, received, accepted, unknown }

enum MessageState {
  sending,
  sent,
  confirmed,
  confirmedByAll,
  receiving,
  received
}

MessageState _messageStateFactory({required MessageStatus status}) {
  switch (status) {
    case MessageStatus.CONFIRMED:
      return MessageState.confirmed;
    case MessageStatus.CONFIRMED_BY_ALL:
      return MessageState.confirmedByAll;
    case MessageStatus.RECEIVED:
      return MessageState.received;
    case MessageStatus.RECEIVING:
      return MessageState.receiving;
    case MessageStatus.SENDING:
      return MessageState.sending;
    case MessageStatus.SENT:
      return MessageState.sent;
  }
  throw UnimplementedError('(_messageStatusFactory) Unmapped status: $status');
}

enum GroupEventContentType {
  none,
  created,
  invited,
  inviteAccepted,
  joined,
  left,
  closed,
  removed
}

GroupEventContentType _groupEventContentTypeFactory({
  required GroupEventType t,
}) {
  switch (t) {
    case GroupEventType.CLOSED:
      return GroupEventContentType.closed;
    case GroupEventType.DEFAULT:
      return GroupEventContentType.none;
    case GroupEventType.INVITED:
      return GroupEventContentType.invited;
    case GroupEventType.JOINED:
      return GroupEventContentType.joined;
    case GroupEventType.LEFT:
      return GroupEventContentType.left;
    case GroupEventType.REMOVED:
      return GroupEventContentType.removed;
    case GroupEventType.CREATED:
      return GroupEventContentType.created;
    case GroupEventType.INVITE_ACCEPTED:
      return GroupEventContentType.inviteAccepted;
  }
  throw UnimplementedError(
      '(_groupEventContentTypeFactory) unmpaped event type: $t');
}

enum ChatRoomStatus { active, inviteAccepted, deactivated }

ChatRoomStatus _chatRoomStatusFactory({required GroupStatus s}) {
  switch (s) {
    case GroupStatus.ACTIVE:
      return ChatRoomStatus.active;
    case GroupStatus.DEACTIVATED:
      return ChatRoomStatus.deactivated;
    case GroupStatus.INVITE_ACCEPTED:
      return ChatRoomStatus.inviteAccepted;
  }
  throw UnimplementedError('(_chatRoomStatusFactory) type: $s');
}
