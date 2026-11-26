import 'package:flutter/material.dart';

/// Display-only data rendered above the chat composer.
@immutable
class ChatFooterPreviewData {
  const ChatFooterPreviewData({
    required this.title,
    required this.content,
    this.leading,
  });

  final String title;
  final String content;
  final Widget? leading;
}

/// A dismissible preview shown above the chat composer.
///
/// This component is controlled by its parent and can represent a reply,
/// attachment, or another pending chat action.
class ChatFooterPreview extends StatelessWidget {
  const ChatFooterPreview({
    super.key,
    required this.data,
    required this.onDismiss,
    this.dismissTooltip,
    this.dismissKey,
    this.surfaceKey,
  });

  final ChatFooterPreviewData data;
  final VoidCallback? onDismiss;
  final String? dismissTooltip;
  final Key? dismissKey;
  final Key? surfaceKey;

  static const _darkBackground = Color(0xFF2C2C2E);
  static const _lightBackground = Color(0xFFE5E5EA);
  static const _darkText = Colors.white;
  static const _lightText = Color(0xFF252525);
  static const _darkClose = Color(0xFFAEAEB2);
  static const _lightClose = Color(0xFF8E8E93);
  static const _horizontalPadding = 12.0;
  static const _verticalPadding = 8.0;
  static const _radius = 3.0;
  static const _closeButtonSize = 32.0;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final textColor = isDark ? _darkText : _lightText;

    return DecoratedBox(
      key: surfaceKey ?? const ValueKey('chat-footer-preview'),
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
            if (data.leading case final leading?) ...[
              Padding(
                padding: const EdgeInsetsDirectional.only(end: 8),
                child: SizedBox.square(
                  dimension: 32,
                  child: Center(child: leading),
                ),
              ),
            ],
            Expanded(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    data.title.trim(),
                    semanticsLabel: data.title.trim(),
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
                key:
                    dismissKey ?? const ValueKey('dismiss-chat-footer-preview'),
                tooltip:
                    dismissTooltip ??
                    MaterialLocalizations.of(context).closeButtonTooltip,
                onPressed: onDismiss,
                icon: Icon(
                  Icons.close_rounded,
                  size: 24,
                  color: isDark ? _darkClose : _lightClose,
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
