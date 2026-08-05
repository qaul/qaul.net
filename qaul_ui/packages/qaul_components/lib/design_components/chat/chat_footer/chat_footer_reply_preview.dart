import 'package:flutter/material.dart';

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

/// Reply target shown above the chat composer.
///
/// This component is controlled by its parent and has no knowledge of message
/// storage, navigation, or sending behavior.
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

  static const _darkBackground = Color(0xFF2C2C2E);
  static const _lightBackground = Color(0xFFE5E5EA);
  static const _darkText = Colors.white;
  static const _lightText = Color(0xFF252525);
  static const _horizontalPadding = 12.0;
  static const _verticalPadding = 8.0;
  static const _radius = 3.0;
  static const _closeButtonSize = 40.0;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;
    final textColor = isDark ? _darkText : _lightText;

    return DecoratedBox(
      key: const ValueKey('chat-footer-reply-preview'),
      decoration: BoxDecoration(
        color: isDark ? _darkBackground : _lightBackground,
        borderRadius: BorderRadius.circular(_radius),
      ),
      child: Padding(
        padding: const EdgeInsetsDirectional.only(
          start: _horizontalPadding,
          top: _verticalPadding,
          bottom: _verticalPadding,
        ),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    data.author.trim(),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontFamily: 'Roboto',
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      height: 1.2,
                      color: textColor,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    data.content.trim(),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontFamily: 'Roboto',
                      fontSize: 14,
                      fontWeight: FontWeight.w400,
                      height: 1.2,
                      color: textColor,
                    ),
                  ),
                ],
              ),
            ),
            SizedBox.square(
              dimension: _closeButtonSize,
              child: IconButton(
                key: const ValueKey('cancel-chat-reply'),
                tooltip:
                    cancelTooltip ??
                    MaterialLocalizations.of(context).closeButtonTooltip,
                onPressed: onCancelReply,
                icon: Icon(
                  Icons.close,
                  size: 24,
                  color: textColor.withValues(alpha: 0.55),
                ),
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints.tightFor(
                  width: _closeButtonSize,
                  height: _closeButtonSize,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
