import 'package:flutter/material.dart';

import '../../l10n/qaul_components_l10n_ext.dart';
import '../../styles/qaul_color_sheet.dart';
import '../users/qaul_avatar.dart';

const Color _kChatHeaderControlColor = Color(0xFF999999);
const double _kChatHeaderDividerWidth = 0.5;

const double kChatHeaderToolbarHeight = kToolbarHeight;
const double kChatHeaderAvatarSize = 40;
const double _kHorizontalPadding = 12;
const double _kBackAvatarGap = 4;
const double _kAvatarTitleGap = 12;
const double _kTitleSubtitleGap = 4;
const double _kHeaderControlIconSize = 30;

class ChatHeaderMenuEntry {
  const ChatHeaderMenuEntry({required this.id, required this.label});

  final String id;
  final String label;
}

Color _chatHeaderShellColor(ThemeData theme) {
  final sheet = QaulColorSheet(theme.brightness);
  return sheet.background;
}

Color _chatHeaderDividerColor(ThemeData theme) {
  return QaulColorSheet(theme.brightness).chatHeaderDivider;
}

Color _chatHeaderTextColor(ThemeData theme) => theme.colorScheme.onSurface;

TextStyle _chatHeaderLineStyle(
  ThemeData theme, {
  required TextStyle? base,
  required double fontSize,
}) {
  return (base ?? const TextStyle()).copyWith(
    fontFamily: 'Roboto',
    fontSize: fontSize,
    fontWeight: FontWeight.w400,
    height: 1.2,
    letterSpacing: 1,
    color: _chatHeaderTextColor(theme),
  );
}

BoxShadow _chatHeaderShadow(ThemeData theme) {
  return theme.brightness == Brightness.dark
      ? const BoxShadow(
          offset: Offset(0, 10),
          blurRadius: 7,
          color: Color(0x66000000),
        )
      : const BoxShadow(blurRadius: 5, color: Color(0x33000000));
}

class _ChatHeaderOverflowMenuButton extends StatelessWidget {
  const _ChatHeaderOverflowMenuButton({
    required this.entries,
    required this.onSelected,
  });

  final List<ChatHeaderMenuEntry> entries;
  final ValueChanged<String> onSelected;

  @override
  Widget build(BuildContext context) {
    return PopupMenuButton<String>(
      tooltip: MaterialLocalizations.of(context).showMenuTooltip,
      onSelected: onSelected,
      itemBuilder: (context) => [
        for (final entry in entries)
          PopupMenuItem<String>(value: entry.id, child: Text(entry.label)),
      ],
      icon: const Icon(
        Icons.more_vert,
        size: _kHeaderControlIconSize,
        color: _kChatHeaderControlColor,
      ),
    );
  }
}

class ChatHeader extends StatelessWidget {
  const ChatHeader({
    super.key,
    required this.onBackPressed,
    this.backButtonTooltip,
    required this.avatar,
    required String displayName,
    required this.isOnline,
    required this.onlineLabel,
    required this.lastSeenLabel,
    this.showOnlineIndicatorWhenOnline = true,
    this.showBackButton = true,
    this.applyTopSafeArea = true,
    this.extraTopPadding = 0,
    this.menuEntries = const [],
    this.onMenuSelected,
  }) : _isGroup = false,
       _primaryTitle = displayName,
       _membersCount = null,
       _formatMembersCount = null;

  const ChatHeader.group({
    super.key,
    required this.onBackPressed,
    this.backButtonTooltip,
    required this.avatar,
    required String groupName,
    required int membersCount,
    String Function(int count)? formatMembersCount,
    this.showBackButton = true,
    this.applyTopSafeArea = true,
    this.extraTopPadding = 0,
    this.menuEntries = const [],
    this.onMenuSelected,
  }) : _isGroup = true,
       _primaryTitle = groupName,
       _membersCount = membersCount,
       _formatMembersCount = formatMembersCount,
       isOnline = false,
       onlineLabel = '',
       lastSeenLabel = '',
       showOnlineIndicatorWhenOnline = false;

  final VoidCallback onBackPressed;
  final String? backButtonTooltip;
  final Widget avatar;
  final bool _isGroup;
  final String _primaryTitle;
  final int? _membersCount;
  final String Function(int count)? _formatMembersCount;
  final bool isOnline;
  final String onlineLabel;
  final String lastSeenLabel;
  final bool showOnlineIndicatorWhenOnline;
  final bool showBackButton;
  final bool applyTopSafeArea;
  final double extraTopPadding;
  final List<ChatHeaderMenuEntry> menuEntries;
  final ValueChanged<String>? onMenuSelected;

  String _subtitle(BuildContext context) {
    if (_isGroup) {
      final formatMembersCount =
          _formatMembersCount ??
          (count) => qaulChatHeaderMembersCountLabel(context, count);
      return formatMembersCount(_membersCount!);
    }
    return isOnline ? onlineLabel : lastSeenLabel;
  }

  bool get _showOnlineDot =>
      !_isGroup && showOnlineIndicatorWhenOnline && isOnline;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final primaryStyle = _chatHeaderLineStyle(
      theme,
      base: theme.textTheme.titleMedium,
      fontSize: 20,
    );
    final secondaryStyle = _chatHeaderLineStyle(
      theme,
      base: theme.textTheme.bodySmall,
      fontSize: 11,
    );
    final subtitle = _subtitle(context);
    final hasSubtitle = subtitle.isNotEmpty;

    final menu = menuEntries.isNotEmpty && onMenuSelected != null
        ? _ChatHeaderOverflowMenuButton(
            entries: menuEntries,
            onSelected: onMenuSelected!,
          )
        : null;

    final header = SizedBox(
      height: kChatHeaderToolbarHeight,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: _kHorizontalPadding),
        child: Row(
          children: [
            if (showBackButton) ...[
              IconButton(
                tooltip:
                    backButtonTooltip ??
                    MaterialLocalizations.of(context).backButtonTooltip,
                onPressed: onBackPressed,
                icon: const Icon(
                  Icons.arrow_back_rounded,
                  size: _kHeaderControlIconSize,
                  color: _kChatHeaderControlColor,
                ),
              ),
              const SizedBox(width: _kBackAvatarGap),
            ],
            _AvatarWithOnlineBadge(
              showOnline: _showOnlineDot,
              child: SizedBox.square(
                dimension: kChatHeaderAvatarSize,
                child: avatar,
              ),
            ),
            const SizedBox(width: _kAvatarTitleGap),
            Expanded(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    _primaryTitle,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: primaryStyle,
                  ),
                  if (hasSubtitle) ...[
                    const SizedBox(height: _kTitleSubtitleGap),
                    Text(
                      subtitle,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: secondaryStyle,
                    ),
                  ],
                ],
              ),
            ),
            if (menu != null) menu,
          ],
        ),
      ),
    );

    final content = applyTopSafeArea
        ? SafeArea(bottom: false, child: header)
        : header;

    return DecoratedBox(
      key: const ValueKey('chat-header-surface'),
      decoration: BoxDecoration(
        color: _chatHeaderShellColor(theme),
        border: Border(
          bottom: BorderSide(
            color: _chatHeaderDividerColor(theme),
            width: _kChatHeaderDividerWidth,
          ),
        ),
        boxShadow: [_chatHeaderShadow(theme)],
      ),
      child: Material(
        color: Colors.transparent,
        child: Padding(
          padding: EdgeInsets.only(top: extraTopPadding),
          child: content,
        ),
      ),
    );
  }
}

class _AvatarWithOnlineBadge extends StatelessWidget {
  const _AvatarWithOnlineBadge({required this.showOnline, required this.child});

  final bool showOnline;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return QaulAvatarBadge(
      size: QaulAvatarSize.small,
      isVisible: showOnline,
      child: child,
    );
  }
}
