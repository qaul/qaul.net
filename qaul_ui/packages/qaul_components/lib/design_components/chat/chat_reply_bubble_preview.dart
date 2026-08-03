import 'package:flutter/material.dart';

/// Display-only data for a replied-message excerpt inside a chat bubble.
@immutable
class ChatReplyPreviewData {
  const ChatReplyPreviewData({required this.author, required this.content});

  final String author;
  final String content;
}

/// The quoted-message block rendered inside a reply bubble.
class ChatReplyBubblePreview extends StatelessWidget {
  const ChatReplyBubblePreview({
    super.key,
    required this.data,
    required this.isOutgoing,
    required this.textScaler,
  });

  final ChatReplyPreviewData data;
  final bool isOutgoing;
  final TextScaler textScaler;

  static const _outgoingColor = Color(0xFF0277BD);
  static const _incomingColor = Color(0xFF303030);
  static const _radius = Radius.circular(3);

  static const _authorStyle = TextStyle(
    fontSize: 14,
    fontWeight: FontWeight.w600,
    height: 1.2,
    letterSpacing: 0,
    color: Colors.white,
  );

  static const _contentStyle = TextStyle(
    fontSize: 14,
    fontWeight: FontWeight.w300,
    height: 1.2,
    letterSpacing: 0,
    color: Colors.white,
  );

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      key: const ValueKey('chat-reply-preview'),
      decoration: BoxDecoration(
        color: isOutgoing ? _outgoingColor : _incomingColor,
        borderRadius: const BorderRadius.all(_radius),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              data.author.trim(),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: _authorStyle,
              textScaler: textScaler,
            ),
            const SizedBox(height: 2),
            Text(
              data.content.trim(),
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
              style: _contentStyle,
              textScaler: textScaler,
            ),
          ],
        ),
      ),
    );
  }
}
