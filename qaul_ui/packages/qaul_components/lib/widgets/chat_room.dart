import 'package:flutter/material.dart';

import 'chat_message.dart';
import 'qaul_chat_bubble.dart';

class ChatRoom extends StatelessWidget {
  const ChatRoom({
    super.key,
    required this.messages,
    this.clock,
    this.padding = const EdgeInsets.all(16),
  });

  final List<ChatMessage> messages;
  final DateTime? clock;
  final EdgeInsetsGeometry padding;

  @override
  Widget build(BuildContext context) {
    final children = <Widget>[];
    var first = true;
    var pendingBubbleRun = <QaulChatBubbleMessage>[];

    void flushBubbleRun() {
      if (pendingBubbleRun.isEmpty) return;

      final displayItems = computeChatBubbleDisplayItems(pendingBubbleRun);
      for (final item in displayItems) {
        children.add(
          Padding(
            padding: EdgeInsets.only(top: first ? 0 : item.marginTop),
            child: QaulChatBubble(
              message: item.message,
              clock: clock ?? DateTime.now(),
              showTimestamp: item.showTimestamp,
            ),
          ),
        );
        first = false;
      }

      pendingBubbleRun = <QaulChatBubbleMessage>[];
    }

    for (final message in messages) {
      if (message is QaulChatBubbleMessage) {
        pendingBubbleRun.add(message);
        continue;
      }

      flushBubbleRun();
      children.add(
        Padding(
          padding: EdgeInsets.only(top: first ? 0 : kChatBubbleSeparatedGap),
          child: message,
        ),
      );
      first = false;
    }

    flushBubbleRun();

    return Padding(
      padding: padding,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: children,
      ),
    );
  }
}
