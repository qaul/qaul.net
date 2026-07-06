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

/// Join metadata with an emphasized member name.
class GroupJoinMetaChatMessage extends ChatMessage {
  const GroupJoinMetaChatMessage({
    required this.id,
    required this.userName,
    required this.joinedSuffix,
  });

  @override
  final String id;

  /// Display name of the member who joined (rendered bold).
  final String userName;

  /// Localized trailing phrase, e.g. "has joined the group".
  final String joinedSuffix;
}

/// Rename metadata shown when a joiner's display name collides with an
/// existing member. Copy is pre-formatted by the caller.
class DuplicateUsernameMetaChatMessage extends ChatMessage {
  const DuplicateUsernameMetaChatMessage({
    required this.id,
    required this.prefix,
    required this.emphasizedName,
    required this.actionLabel,
  });

  @override
  final String id;

  /// Localized text before the emphasized name.
  final String prefix;

  /// The disambiguated display name, rendered bold.
  final String emphasizedName;

  /// Localized action label, e.g. "Edit usernames".
  final String actionLabel;
}
