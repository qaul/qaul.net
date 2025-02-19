part of '../chat_room.dart';

abstract class MessageContent extends Equatable {
  const MessageContent();

  factory MessageContent.fromBuffer(List<int> buffer) {
    final content = ChatContentMessage.fromBuffer(buffer);
    switch (content.whichMessage()) {
      case ChatContentMessage_Message.chatContent:
        final message = content.ensureChatContent();
        return TextMessageContent(message.text);
      case ChatContentMessage_Message.fileContent:
        final message = content.ensureFileContent();
        // TODO: historyIndex is not provided
        return FileShareContent(
          historyIndex: 0,
          fileId: message.fileId.toStringUnsigned(),
          fileName: message.fileName,
          size: message.fileSize,
          description: message.fileDescription,
        );
      case ChatContentMessage_Message.groupEvent:
        final message = content.ensureGroupEvent();
        return GroupEventContent(
          userId: Uint8List.fromList(message.userId),
          type: _groupEventContentTypeFactory(t: message.eventType),
        );
      case ChatContentMessage_Message.notSet:
        // TODO: log warning
        return NoneMessageContent();
    }
  }
}

class NoneMessageContent extends MessageContent {
  @override
  List<Object?> get props => [];
}

class TextMessageContent extends MessageContent {
  const TextMessageContent(this.content);

  final String content;

  @override
  List<Object?> get props => [content];
}

class GroupEventContent extends MessageContent {
  GroupEventContent({
    required this.userId,
    required this.type,
  }) : userIdBase58 = Base58Encode(userId);

  final Uint8List userId;
  final GroupEventContentType type;

  final String userIdBase58;

  String toEventMessage(User user) {
    String msgBase(String event) => '"${user.name}" has $event the group';

    switch (type) {
      case GroupEventContentType.none:
        return '';
      case GroupEventContentType.invited:
        return msgBase('been invited to');
      case GroupEventContentType.joined:
        return msgBase('joined');
      case GroupEventContentType.left:
        return msgBase('left');
      case GroupEventContentType.closed:
        return 'The group has been closed';
      case GroupEventContentType.removed:
        return msgBase('been removed from');
      case GroupEventContentType.created:
        return 'The group has been created';
      case GroupEventContentType.inviteAccepted:
        return '"${user.name}" has accepted the invite.';
    }
  }

  @override
  List<Object?> get props => [userIdBase58, type];
}

class FileShareContent extends MessageContent with FilePathResolverMixin {
  const FileShareContent({
    required this.historyIndex,
    required this.fileId,
    required this.fileName,
    required this.size,
    required this.description,
  });

  final int historyIndex;
  final String fileId;
  final String fileName;
  final int size;
  final String description;

  @override
  List<Object?> get props => [fileId, fileName];

  String get extension => fileName.split('.').last;

  String filePath(WidgetRef ref) =>
      getFilePath(ref, id: fileId, extension: extension);
}
