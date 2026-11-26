import 'package:flutter/material.dart';

import 'chat_footer_preview.dart';

/// Display-only data for the message currently selected as a reply target.
@immutable
class ChatFooterReplyPreviewData {
  const ChatFooterReplyPreviewData({
    required this.author,
    required this.content,
  });

  final String author;
  final String content;
}

/// Backwards-compatible reply-specific wrapper around [ChatFooterPreview].
class ChatFooterReplyPreview extends StatelessWidget {
  const ChatFooterReplyPreview({
    super.key,
    required this.data,
    required this.onCancelReply,
    this.cancelTooltip,
  });

  final ChatFooterReplyPreviewData data;
  final VoidCallback? onCancelReply;
  final String? cancelTooltip;

  @override
  Widget build(BuildContext context) {
    return ChatFooterPreview(
      surfaceKey: const ValueKey('chat-footer-reply-preview'),
      data: ChatFooterPreviewData(title: data.author, content: data.content),
      onDismiss: onCancelReply,
      dismissTooltip: cancelTooltip,
      dismissKey: const ValueKey('cancel-chat-reply'),
    );
  }
}
