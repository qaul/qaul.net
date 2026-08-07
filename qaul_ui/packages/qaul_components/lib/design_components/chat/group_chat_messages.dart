import 'package:flutter/material.dart';
import 'package:utils/utils.dart';

import 'message_presentation_meta.dart';
import 'qaul_chat_bubble.dart';

@immutable
class QaulGroupMessageSender {
  const QaulGroupMessageSender({
    required this.idBase58,
    required this.name,
    this.isConnected = false,
  });

  final String idBase58;
  final String name;

  final bool isConnected;
}

@immutable
class MessagePresentation {
  const MessagePresentation({
    required this.messageId,
    required this.isPrimary,
    required this.sender,
    required this.meta,
    required this.bubbleDisplay,
  });

  factory MessagePresentation.fromComputation({
    required String messageId,
    required QaulGroupMessageSender? sender,
    required MessagePresentationComputation computation,
  }) {
    return MessagePresentation(
      messageId: messageId,
      isPrimary: computation.isPrimary,
      sender: sender,
      meta: computation.meta,
      bubbleDisplay: computation.bubbleDisplay,
    );
  }

  final String messageId;
  final bool isPrimary;
  final QaulGroupMessageSender? sender;
  final MessagePresentationMeta meta;
  final QaulChatBubbleDisplayItem? bubbleDisplay;
}

class ChatMessageRenderer {
  static Widget renderText({
    required MessagePresentation presentation,
    required ChatRenderMode mode,
    required DateTime clock,
    bool isSelected = false,
  }) {
    final display = presentation.bubbleDisplay;
    if (display == null) return const SizedBox.shrink();

    if (mode == ChatRenderMode.group && !presentation.isPrimary) {
      return GroupTextMessageItem(
        presentation: presentation,
        clock: clock,
        isSelected: isSelected,
      );
    }

    return DirectTextMessageItem(
      presentation: presentation,
      clock: clock,
      horizontalGutter: true,
      isSelected: isSelected,
    );
  }

  static Widget renderMedia({
    required Widget child,
    required MessagePresentation presentation,
    required ChatRenderMode mode,
    required DateTime clock,
    bool isSelected = false,
  }) {
    final display = presentation.bubbleDisplay;
    if (display == null) return child;

    final mediaBubble = QaulChatBubble.media(
      message: display.message,
      clock: clock,
      showTimestamp: presentation.meta.showTimestamp,
      isSelected: isSelected,
      child: child,
    );

    if (mode != ChatRenderMode.group || presentation.isPrimary) {
      return Padding(
        padding: EdgeInsetsDirectional.only(
          top: presentation.meta.topSpacing,
          end: presentation.isPrimary ? 16 : 0,
          start: presentation.isPrimary ? 0 : 16,
        ),
        child: mediaBubble,
      );
    }

    return GroupMessageShell(
      marginTop: presentation.meta.topSpacing,
      sender: presentation.sender,
      showSenderName: presentation.meta.showSenderName,
      showSenderAvatar: presentation.meta.showAvatar,
      child: mediaBubble,
    );
  }
}

class DirectTextMessageItem extends StatelessWidget {
  const DirectTextMessageItem({
    super.key,
    required this.presentation,
    required this.clock,
    this.horizontalGutter = true,
    this.isSelected = false,
  });

  final MessagePresentation presentation;
  final DateTime clock;
  final bool isSelected;

  /// When true, applies a 16px gutter on the trailing edge for primary
  /// (outgoing) bubbles and on the leading edge for secondary (incoming).
  /// Set to false only for layouts that intentionally render flush.
  final bool horizontalGutter;

  @override
  Widget build(BuildContext context) {
    final display = presentation.bubbleDisplay!;
    final isPrimary = presentation.isPrimary;
    return Padding(
      padding: EdgeInsetsDirectional.only(
        top: presentation.meta.topSpacing,
        start: horizontalGutter && !isPrimary ? 16 : 0,
        end: horizontalGutter && isPrimary ? 16 : 0,
      ),
      child: QaulChatBubble(
        message: display.message,
        clock: clock,
        showTimestamp: presentation.meta.showTimestamp,
        isSelected: isSelected,
      ),
    );
  }
}

class GroupTextMessageItem extends StatelessWidget {
  const GroupTextMessageItem({
    super.key,
    required this.presentation,
    required this.clock,
    this.isSelected = false,
  });

  final MessagePresentation presentation;
  final DateTime clock;
  final bool isSelected;

  @override
  Widget build(BuildContext context) {
    final display = presentation.bubbleDisplay!;
    final sender = presentation.sender;
    final senderNameColor = sender == null
        ? null
        : colorGenerationStrategy(sender.idBase58);
    final bubbleMessage = presentation.meta.showSenderName && sender != null
        ? display.message.copyWith(
            senderDisplayName: sender.name,
            senderDisplayNameColor: senderNameColor,
          )
        : display.message;

    return GroupMessageShell(
      marginTop: presentation.meta.topSpacing,
      sender: sender,
      showSenderName: false,
      showSenderAvatar: presentation.meta.showAvatar,
      child: QaulChatBubble(
        message: bubbleMessage,
        clock: clock,
        showTimestamp: presentation.meta.showTimestamp,
        isSelected: isSelected,
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
    final textScaler = chatBubbleTextScaler(context);
    final senderNameColor = sender == null
        ? null
        : colorGenerationStrategy(sender!.idBase58);
    return Padding(
      padding: EdgeInsetsDirectional.only(top: marginTop, start: 12),
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
                      style: kGroupSenderNameTextStyle.copyWith(
                        color: senderNameColor,
                      ),
                      textScaler: textScaler,
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
    final textScaler = chatBubbleTextScaler(context);

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
              textScaler: textScaler,
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
