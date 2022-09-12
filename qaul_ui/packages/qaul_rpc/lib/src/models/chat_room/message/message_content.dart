part of '../chat_room.dart';


abstract class MessageContent extends Equatable {
  const MessageContent();

  factory MessageContent.fromBuffer(List<int> buffer) {
    final content = ChatContentMessage.fromBuffer(buffer);
    switch (content.whichMessage()) {
      case ChatContentMessage_Message.chatContent:
        return TextMessageContent(String.fromCharCodes(buffer));
      case ChatContentMessage_Message.fileContent:
        final file = FileSharingContainer.fromBuffer(buffer).fileInfo;
        return FileShareContent(
          historyIndex: file.startIndex,
          fileId: file.fileId.toStringUnsigned(),
          fileName: file.fileName,
          size: file.fileSize,
          description: file.fileDescr,
        );
      case ChatContentMessage_Message.groupEvent:
        final event = GroupEvent.fromBuffer(buffer);
        return GroupEventContent(
          userId: Uint8List.fromList(event.userId),
          type: _groupEventContentTypeFactory(t: event.eventType),
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
  const GroupEventContent({
    required this.userId,
    required this.type,
  });

  final Uint8List userId;
  final GroupEventContentType type;

  String get userIdBase58 => Base58Encode(userId);

  @override
  List<Object?> get props => [userIdBase58, type];

  GroupEventContent.fromJson(Map<String, dynamic> json)
      : userId = Uint8List.fromList(json['userId']),
        type = _typeFromString(json['type']);

  Map<String, dynamic> toJson() {
    return {'userId': userId.toList(), 'type': type.toString()};
  }

  static GroupEventContentType _typeFromString(String s) {
    for (var element in GroupEventContentType.values) {
      if (element.toString() == s) return element;
    }
    throw ArgumentError.value(s, 'GroupEventType', 'unable to deserialize');
  }
}

class FileShareContent extends MessageContent {
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

  String filePath(Reader read) {
    var storagePath = read(libqaulLogsStoragePath)!.replaceAll('/logs', '');
    var uuid = read(defaultUserProvider)!.idBase58;

    return '$storagePath/$uuid/files/$fileId.$extension';
  }
}
