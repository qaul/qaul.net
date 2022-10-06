part of '../chat_room.dart';

@immutable
class Message with EquatableMixin implements Comparable<Message> {
  Message({
    required this.senderId,
    required this.messageId,
    required this.content,
    required this.index,
    required this.sentAt,
    required this.receivedAt,
    this.status = MessageState.sending,
  }) : messageIdBase58 = Base58Encode(messageId);

  final Uint8List senderId;
  final Uint8List messageId;
  final int index;
  final MessageState status;
  final DateTime sentAt;
  final DateTime receivedAt;
  final MessageContent content;

  final String messageIdBase58;

  factory Message.fromChatMessage(ChatMessage m) {
    return Message(
      senderId: Uint8List.fromList(m.senderId),
      messageId: Uint8List.fromList(m.messageId),
      content: MessageContent.fromBuffer(m.content),
      index: m.index.toInt(),
      status: _messageStateFactory(status: m.status),
      sentAt: DateTime.fromMillisecondsSinceEpoch(m.sentAt.toInt()),
      receivedAt: DateTime.fromMillisecondsSinceEpoch(m.receivedAt.toInt()),
    );
  }

  @override
  int compareTo(dynamic other) {
    assert(
      runtimeType == other.runtimeType,
      "The sorting algorithm must not compare incomparable keys, since they don't "
      'know how to order themselves relative to each other. Comparing $this with $other',
    );
    return (other as Message).sentAt.compareTo(sentAt);
  }

  @override
  List<Object?> get props => [senderId, messageId, content];
}
