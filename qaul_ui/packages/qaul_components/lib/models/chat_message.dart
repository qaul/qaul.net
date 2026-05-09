import '../design_components/chat/qaul_chat_bubble.dart' show MessageStatus;
import 'chat_user.dart';

/// Sealed hierarchy of messages a chat timeline renders.
sealed class ChatMessage {
  const ChatMessage();
  String get id;
}

/// A text message bubble.
class TextChatMessage extends ChatMessage {
  const TextChatMessage({
    required this.id,
    required this.sender,
    required this.content,
    required this.sentAt,
    required this.receivedAt,
    required this.status,
  });

  @override
  final String id;
  final ChatUser sender;
  final String content;
  final DateTime sentAt;
  final DateTime receivedAt;
  final MessageStatus status;
}

/// Caller-supplied event meta, pre-formatted for i18n.
/// Examples: "Alice joined the group", "Encryption is now on".
class MetaChatMessage extends ChatMessage {
  const MetaChatMessage({required this.id, required this.label});

  @override
  final String id;
  final String label;
}
