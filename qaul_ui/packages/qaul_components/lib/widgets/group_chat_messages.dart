import 'package:flutter/material.dart';
import 'package:utils/utils.dart';

import 'qaul_chat_bubble.dart';

/// Lightweight sender snapshot for group chat UI (decouples from RPC `User`).
@immutable
class QaulGroupMessageSender {
  const QaulGroupMessageSender({
    required this.idBase58,
    required this.name,
    this.isConnected = false,
  });

  final String idBase58;
  final String name;

  /// When true, [GroupSenderAvatar] shows the online indicator.
  final bool isConnected;
}

enum ChatRenderMode { direct, group }

/// Name + avatar visibility for an incoming group text bubble, given run context.
@immutable
class GroupIncomingBubbleFlags {
  const GroupIncomingBubbleFlags({
    required this.showSenderName,
    required this.showSenderAvatar,
  });

  final bool showSenderName;
  final bool showSenderAvatar;

  static GroupIncomingBubbleFlags fromSequentialMessages({
    required QaulChatBubbleDisplayItem? textDisplay,
    required bool prevSameSender,
    required bool nextSameSender,
  }) {
    final showSenderName = textDisplay != null
        ? textDisplay.message.edges.contains(TailEdge.bottomStart) &&
            !textDisplay.message.edges.contains(TailEdge.topStart)
        : !prevSameSender;
    final showSenderAvatar = textDisplay != null
        ? textDisplay.showTimestamp
        : !nextSameSender;
    return GroupIncomingBubbleFlags(
      showSenderName: showSenderName,
      showSenderAvatar: showSenderAvatar,
    );
  }
}

@immutable
class MessagePresentation {
  const MessagePresentation({
    required this.messageId,
    required this.isPrimary,
    required this.sender,
    required this.showSenderName,
    required this.showSenderAvatar,
    required this.bubbleDisplay,
  });

  /// Same grouping rules as the chat screen, for flat lists (e.g. Widgetbook
  /// previews) so [ChatMessageRenderer.renderText] is the single render path.
  factory MessagePresentation.forSequentialTextBubble({
    required QaulChatBubbleDisplayItem item,
    required int index,
    required List<QaulChatBubbleDisplayItem> items,
    required bool isPrimary,
    required QaulGroupMessageSender? sender,
  }) {
    if (isPrimary) {
      return MessagePresentation(
        messageId: 'seq-$index',
        isPrimary: true,
        sender: null,
        showSenderName: false,
        showSenderAvatar: false,
        bubbleDisplay: item,
      );
    }
    final prev = index > 0 ? items[index - 1] : null;
    final next = index < items.length - 1 ? items[index + 1] : null;
    final prevSame = prev != null &&
        prev.message.senderIdBase58 == item.message.senderIdBase58;
    final nextSame = next != null &&
        next.message.senderIdBase58 == item.message.senderIdBase58;
    final flags = GroupIncomingBubbleFlags.fromSequentialMessages(
      textDisplay: item,
      prevSameSender: prevSame,
      nextSameSender: nextSame,
    );
    return MessagePresentation(
      messageId: 'seq-$index',
      isPrimary: false,
      sender: sender,
      showSenderName: flags.showSenderName,
      showSenderAvatar: flags.showSenderAvatar,
      bubbleDisplay: item,
    );
  }

  final String messageId;
  final bool isPrimary;
  final QaulGroupMessageSender? sender;
  final bool showSenderName;
  final bool showSenderAvatar;
  final QaulChatBubbleDisplayItem? bubbleDisplay;
}

class ChatMessageRenderer {
  static Widget renderText({
    required MessagePresentation presentation,
    required ChatRenderMode mode,
    required DateTime clock,
  }) {
    final display = presentation.bubbleDisplay;
    if (display == null) return const SizedBox.shrink();

    if (mode == ChatRenderMode.group && !presentation.isPrimary) {
      return GroupTextMessageItem(
        display: display,
        clock: clock,
        sender: presentation.sender,
        showSenderName: presentation.showSenderName,
        showSenderAvatar: presentation.showSenderAvatar,
      );
    }

    return DirectTextMessageItem(
      display: display,
      clock: clock,
    );
  }

  static Widget wrapNonText({
    required Widget child,
    required MessagePresentation presentation,
    required ChatRenderMode mode,
  }) {
    if (mode != ChatRenderMode.group || presentation.isPrimary) {
      return child;
    }

    return GroupMessageShell(
      marginTop: presentation.bubbleDisplay?.marginTop ?? 0,
      sender: presentation.sender,
      showSenderName: false,
      showSenderAvatar: presentation.showSenderAvatar,
      child: child,
    );
  }
}

class DirectTextMessageItem extends StatelessWidget {
  const DirectTextMessageItem({
    super.key,
    required this.display,
    required this.clock,
  });

  final QaulChatBubbleDisplayItem display;
  final DateTime clock;

  @override
  Widget build(BuildContext context) {
    final isPrimary = display.message.messageType == MessageType.primary;
    return Padding(
      padding: EdgeInsetsDirectional.only(
        top: display.marginTop,
        start: isPrimary ? 0 : 16,
        end: isPrimary ? 16 : 0,
      ),
      child: QaulChatBubble(
        message: display.message,
        clock: clock,
        showTimestamp: display.showTimestamp,
      ),
    );
  }
}

class GroupTextMessageItem extends StatelessWidget {
  const GroupTextMessageItem({
    super.key,
    required this.display,
    required this.clock,
    required this.sender,
    required this.showSenderName,
    required this.showSenderAvatar,
  });

  final QaulChatBubbleDisplayItem display;
  final DateTime clock;
  final QaulGroupMessageSender? sender;
  final bool showSenderName;
  final bool showSenderAvatar;

  @override
  Widget build(BuildContext context) {
    final senderNameColor =
        sender == null ? null : colorGenerationStrategy(sender!.idBase58);
    final bubbleMessage = showSenderName && sender != null
        ? display.message.copyWith(
            senderDisplayName: sender!.name,
            senderDisplayNameColor: senderNameColor,
          )
        : display.message;

    return GroupMessageShell(
      marginTop: display.marginTop,
      sender: sender,
      showSenderName: false,
      showSenderAvatar: showSenderAvatar,
      child: QaulChatBubble(
        message: bubbleMessage,
        clock: clock,
        showTimestamp: display.showTimestamp,
      ),
    );
  }
}

class GroupMessageShell extends StatelessWidget {
  const GroupMessageShell({
    super.key,
    required this.marginTop,
    required this.sender,
    required this.showSenderName,
    required this.showSenderAvatar,
    required this.child,
  });

  final double marginTop;
  final QaulGroupMessageSender? sender;
  final bool showSenderName;
  final bool showSenderAvatar;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final senderNameColor =
        sender == null ? null : colorGenerationStrategy(sender!.idBase58);
    return Padding(
      padding: EdgeInsetsDirectional.only(
        top: marginTop,
        start: 16,
        end: 0,
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          SizedBox(
            width: 40,
            child: showSenderAvatar && sender != null
                ? Align(
                    alignment: Alignment.bottomCenter,
                    child: GroupSenderAvatar(sender: sender!),
                  )
                : const SizedBox.shrink(),
          ),
          const SizedBox(width: 8),
          Flexible(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (showSenderName && sender != null)
                  Padding(
                    padding: const EdgeInsets.only(bottom: 4),
                    child: Text(
                      sender!.name,
                      style: TextStyle(
                        fontFamily: 'Roboto',
                        fontSize: 11,
                        fontWeight: FontWeight.w400,
                        height: 1.25,
                        letterSpacing: 0.5,
                        color: senderNameColor,
                      ),
                    ),
                  ),
                child,
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class GroupSenderAvatar extends StatelessWidget {
  const GroupSenderAvatar({super.key, required this.sender});

  final QaulGroupMessageSender sender;

  @override
  Widget build(BuildContext context) {
    final background = colorGenerationStrategy(sender.idBase58);

    return SizedBox(
      width: 28,
      height: 28,
      child: Stack(
        clipBehavior: Clip.none,
        children: [
          CircleAvatar(
            radius: 14,
            backgroundColor: background,
            child: Text(
              initials(sender.name),
              style: const TextStyle(
                fontFamily: 'Roboto',
                fontSize: 14,
                color: Colors.white,
                fontWeight: FontWeight.w300,
                height: 1.2,
                letterSpacing: 0,
              ),
            ),
          ),
          if (sender.isConnected)
            Positioned(
              right: -1,
              bottom: -1,
              child: Container(
                width: 9,
                height: 9,
                decoration: BoxDecoration(
                  color: Colors.greenAccent.shade700,
                  shape: BoxShape.circle,
                  border: Border.all(color: Colors.white, width: 0.5),
                ),
              ),
            ),
        ],
      ),
    );
  }
}
