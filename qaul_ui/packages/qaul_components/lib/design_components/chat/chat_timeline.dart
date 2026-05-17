import 'package:flutter/material.dart';

import '../../models/chat_message.dart' as model;
import '../../models/chat_user.dart';
import 'compute_message_presentation.dart';
import 'group_chat_messages.dart';
import 'message_presentation_meta.dart';
import 'qaul_chat_bubble.dart';
import 'room_meta_message.dart';

/// A design-system-friendly chat timeline widget.
///
/// Consumers own the scroll context; this widget renders a plain [Column].
/// Use [ChatTimeline.direct] for 1-1 chats and [ChatTimeline.group] for
/// group chats.
class ChatTimeline extends StatelessWidget {
  /// Creates a direct (1-to-1) chat timeline.
  const ChatTimeline.direct({
    super.key,
    required this.currentUser,
    required this.messages,
    this.clock,
    this.padding = const EdgeInsets.all(16),
  }) : _mode = ChatRenderMode.direct;

  /// Creates a group chat timeline. Sender identity is derived from each
  /// [TextChatMessage.sender].
  const ChatTimeline.group({
    super.key,
    required this.currentUser,
    required this.messages,
    this.clock,
    this.padding = const EdgeInsets.all(16),
  }) : _mode = ChatRenderMode.group;

  final ChatUser currentUser;
  final List<model.ChatMessage> messages;

  /// Reference instant used for relative time labels ("Now", "N min").
  /// Defaults to [DateTime.now] if not provided.
  final DateTime? clock;

  /// Outer inset around the timeline column (matches legacy [ChatRoom]).
  final EdgeInsetsGeometry padding;

  final ChatRenderMode _mode;

  // ---------------------------------------------------------------------------
  // Helpers
  // ---------------------------------------------------------------------------

  bool _isOutgoing(model.TextChatMessage msg) =>
      msg.sender.id == currentUser.id;

  QaulGroupMessageSender _groupSender(model.TextChatMessage msg) {
    final sender = msg.sender;
    return QaulGroupMessageSender(idBase58: sender.id, name: sender.name);
  }

  // ---------------------------------------------------------------------------
  // Build helpers
  // ---------------------------------------------------------------------------

  /// Translates the model list into a flat list of [_TimelineEntry] items,
  /// synthesising date-dividers between [TextChatMessage] entries whose
  /// calendar days differ (and one before the first text message).
  List<_TimelineEntry> _buildEntries() {
    final entries = <_TimelineEntry>[];
    DateTime? lastTextDay;

    for (final msg in messages) {
      if (msg is model.TextChatMessage) {
        final day =
            DateTime(msg.sentAt.year, msg.sentAt.month, msg.sentAt.day);

        if (lastTextDay == null || !_sameDay(lastTextDay, day)) {
          entries.add(_DateDividerEntry(date: day));
          lastTextDay = day;
        }
        entries.add(_TextEntry(message: msg));
      } else if (msg is model.MetaChatMessage) {
        entries.add(_MetaEntry(message: msg));
      }
    }

    return entries;
  }

  static bool _sameDay(DateTime a, DateTime b) =>
      a.year == b.year && a.month == b.month && a.day == b.day;

  // ---------------------------------------------------------------------------
  // build
  // ---------------------------------------------------------------------------

  @override
  Widget build(BuildContext context) {
    final effectiveClock = clock ?? DateTime.now();
    final entries = _buildEntries();

    // Build the ascending timeline rows needed by computeChatMessagePresentation.
    // We only put TextChatMessage entries into the timeline (date dividers and
    // MetaChatMessage have no position in the bubble-linking algorithm).
    final textEntries = entries.whereType<_TextEntry>().toList();

    final rows = <ChatTimelinePresentationRow>[
      for (final e in textEntries)
        ChatTimelinePresentationRow(
          messageIdBase58: e.message.id,
          senderIdBase58: e.message.sender.id,
          sentAt: e.message.sentAt,
          isText: true,
          isOutgoing: _isOutgoing(e.message),
          qaulBubbleBaseWithoutLayout: QaulChatBubbleMessage(
            content: e.message.content,
            sentAt: e.message.sentAt,
            receivedAt: e.message.receivedAt,
            status: e.message.status,
            messageType: _isOutgoing(e.message)
                ? MessageType.primary
                : MessageType.secondary,
            edges: const [],
            senderIdBase58: e.message.sender.id,
          ),
        ),
    ];

    final computedMap = computeChatMessagePresentation(
      ascendingTimeline: rows,
      layoutMode: _mode,
    );

    final children = <Widget>[];
    var isFirst = true;

    for (final entry in entries) {
      switch (entry) {
        case _DateDividerEntry(:final date):
          final topPad = isFirst ? 0.0 : kChatBubbleSeparatedGap;
          children.add(
            Padding(
              padding: EdgeInsets.only(top: topPad),
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 16),
                child: RoomMetaMessage.date(date: date),
              ),
            ),
          );
          isFirst = false;

        case _MetaEntry(:final message):
          final topPad = isFirst ? 0.0 : kChatBubbleSeparatedGap;
          final metaStyle = Theme.of(context).textTheme.bodySmall?.copyWith(
                fontSize: 12,
                height: 1.2,
                color: Theme.of(context)
                    .colorScheme
                    .onSurface
                    .withValues(alpha: 0.55),
              );
          children.add(
            Padding(
              padding: EdgeInsets.only(top: topPad),
              child: Center(
                child: Text(
                  message.label,
                  style: metaStyle,
                ),
              ),
            ),
          );
          isFirst = false;

        case _TextEntry(:final message):
          final computation = computedMap[message.id];
          if (computation == null) break;

          final isOutgoing = _isOutgoing(message);

          QaulGroupMessageSender? sender;
          if (!isOutgoing) {
            sender = _groupSender(message);
          }

          final presentation = MessagePresentation.fromComputation(
            messageId: message.id,
            sender: sender,
            computation: computation,
          );

          children.add(
            ChatMessageRenderer.renderText(
              presentation: presentation,
              mode: _mode,
              clock: effectiveClock,
            ),
          );
          isFirst = false;
      }
    }

    return Padding(
      padding: padding,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: children,
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Private sealed timeline entry types
// ---------------------------------------------------------------------------

sealed class _TimelineEntry {
  const _TimelineEntry();
}

class _TextEntry extends _TimelineEntry {
  const _TextEntry({required this.message});
  final model.TextChatMessage message;
}

class _MetaEntry extends _TimelineEntry {
  const _MetaEntry({required this.message});
  final model.MetaChatMessage message;
}

class _DateDividerEntry extends _TimelineEntry {
  const _DateDividerEntry({required this.date});
  final DateTime date;
}
