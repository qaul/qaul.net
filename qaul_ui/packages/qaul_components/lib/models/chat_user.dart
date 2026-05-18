import 'package:flutter/widgets.dart';

/// A user as the chat timeline sees them.
///
/// `id` is opaque to the design system — callers may use any stable string
/// (e.g. base58, email, hash). Used only for grouping consecutive messages
/// from the same sender.
class ChatUser {
  const ChatUser({
    required this.id,
    required this.name,
    this.avatar,
  });

  final String id;
  final String name;

  /// When null, ChatTimeline renders an initials avatar per design rules.
  final ImageProvider? avatar;
}
