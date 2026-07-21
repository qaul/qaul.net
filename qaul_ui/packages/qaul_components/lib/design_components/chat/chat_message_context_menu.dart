import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

const _kComponentsPackage = 'qaul_components';
const _kNormalColor = Color(0xFF999999);
const _kDisabledDarkColor = Color(0xFF5F5F5F);
const _kDisabledLightColor = Color(0xFFC7C7C7);
const _kMenuPageSize = 5;

/// Asset paths for the actions supplied with [ChatMessageContextMenu].
abstract final class ChatMessageContextMenuIcons {
  static const reply = 'assets/icons/sub_menu/reply_arrow.svg';
  static const forward = 'assets/icons/sub_menu/forward_arrow.svg';
  static const edit = 'assets/icons/sub_menu/edit.svg';
  static const info = 'assets/icons/sub_menu/info.svg';
  static const share = 'assets/icons/sub_menu/share.svg';
  static const copy = 'assets/icons/sub_menu/copy.svg';
  static const delete = 'assets/icons/sub_menu/delete.svg';
}

/// A quick reaction displayed inside a [ChatMessageReactionRow].
class ChatMessageQuickReaction {
  const ChatMessageQuickReaction({
    required this.child,
    required this.semanticLabel,
    this.onPressed,
    this.enabled = true,
  });

  final Widget child;
  final String semanticLabel;
  final VoidCallback? onPressed;
  final bool enabled;
}

/// Base configuration shared by every paginated menu element.
sealed class ChatMessageContextMenuElement {
  const ChatMessageContextMenuElement({
    this.enabled = true,
    this.hidden = false,
  });

  /// Disabled elements remain visible but cannot be activated or highlighted.
  final bool enabled;

  /// Hidden elements do not occupy a slot in pagination or layout.
  final bool hidden;
}

/// A single menu line containing quick reactions and an optional add button.
class ChatMessageReactionRow extends ChatMessageContextMenuElement {
  const ChatMessageReactionRow({
    required this.reactions,
    this.onAddReaction,
    this.showAddReaction = true,
    super.enabled,
    super.hidden,
  });

  final List<ChatMessageQuickReaction> reactions;
  final VoidCallback? onAddReaction;
  final bool showAddReaction;
}

/// A single labelled action line in a [ChatMessageContextMenu].
class ChatMessageContextMenuAction extends ChatMessageContextMenuElement {
  const ChatMessageContextMenuAction({
    required this.id,
    required this.label,
    required this.iconAsset,
    this.onPressed,
    super.enabled,
    super.hidden,
  });

  const ChatMessageContextMenuAction.reply({
    this.onPressed,
    super.enabled,
    super.hidden,
  }) : id = 'reply',
       label = 'Reply',
       iconAsset = ChatMessageContextMenuIcons.reply;

  const ChatMessageContextMenuAction.forward({
    this.onPressed,
    super.enabled,
    super.hidden,
  }) : id = 'forward',
       label = 'Forward',
       iconAsset = ChatMessageContextMenuIcons.forward;

  /// Creates Edit with its dedicated [onEdit] callback.
  const ChatMessageContextMenuAction.edit({
    VoidCallback? onEdit,
    super.enabled,
    super.hidden,
  }) : id = 'edit',
       label = 'Edit',
       iconAsset = ChatMessageContextMenuIcons.edit,
       onPressed = onEdit;

  final String id;
  final String label;
  final String iconAsset;
  final VoidCallback? onPressed;
}

/// Contextual actions for a selected chat message.
///
/// The menu owns presentation and local pagination only. It has no knowledge
/// of message storage, navigation, or backend behavior. A parent can position
/// it with an [Overlay], [Stack], or any other appropriate layout.
class ChatMessageContextMenu extends StatefulWidget {
  const ChatMessageContextMenu({super.key, required this.elements});

  final List<ChatMessageContextMenuElement> elements;

  static const double width = 216;

  @override
  State<ChatMessageContextMenu> createState() => _ChatMessageContextMenuState();
}

class _ChatMessageContextMenuState extends State<ChatMessageContextMenu> {
  int _pageIndex = 0;

  @override
  void didUpdateWidget(ChatMessageContextMenu oldWidget) {
    super.didUpdateWidget(oldWidget);
    final pageCount = _buildMenuPages(_visibleElements()).length;
    if (_pageIndex >= pageCount) _pageIndex = pageCount - 1;
  }

  List<ChatMessageContextMenuElement> _visibleElements() => widget.elements
      .where((element) => !element.hidden)
      .toList(growable: false);

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final surface = isDark ? const Color(0xFF282828) : const Color(0xFFF1F1F1);
    final reactionSurface = isDark
        ? const Color(0xFF5A5A5A)
        : const Color(0xFFE0E0E0);
    final activeColor = isDark ? Colors.white : const Color(0xFF252525);
    final disabledColor = isDark ? _kDisabledDarkColor : _kDisabledLightColor;
    final pages = _buildMenuPages(_visibleElements());
    final pageIndex = _pageIndex.clamp(0, pages.length - 1);
    final page = pages[pageIndex];

    final rows = <Widget>[
      if (page.hasPrevious)
        _NavigationRow(
          key: const ValueKey('previous-page'),
          asset: 'assets/icons/sub_menu/arrow-up.svg',
          semanticLabel: 'Previous page',
          onPressed: () => setState(() => _pageIndex = pageIndex - 1),
          activeColor: activeColor,
          disabledColor: disabledColor,
        ),
      for (final element in page.elements)
        switch (element) {
          ChatMessageReactionRow() => _ReactionRow(
            key: const ValueKey('reaction-row'),
            row: element,
            surface: reactionSurface,
            activeColor: activeColor,
            disabledColor: disabledColor,
          ),
          ChatMessageContextMenuAction() => _MessageAction(
            key: ValueKey(element.id),
            action: element,
            activeColor: activeColor,
            disabledColor: disabledColor,
          ),
        },
      if (page.hasNext)
        _NavigationRow(
          key: const ValueKey('next-page'),
          asset: 'assets/icons/sub_menu/arrow-down.svg',
          semanticLabel: 'Next page',
          onPressed: () => setState(() => _pageIndex = pageIndex + 1),
          activeColor: activeColor,
          disabledColor: disabledColor,
        ),
    ];

    return Material(
      color: surface,
      elevation: 8,
      shadowColor: Colors.black.withValues(alpha: 0.35),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(18),
        side: const BorderSide(color: _kNormalColor, width: 1),
      ),
      clipBehavior: Clip.antiAlias,
      child: SizedBox(
        width: ChatMessageContextMenu.width,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(10, 12, 10, 14),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (var index = 0; index < rows.length; index++) ...[
                if (index > 0) const SizedBox(height: 2),
                rows[index],
              ],
            ],
          ),
        ),
      ),
    );
  }
}

List<_MenuPage> _buildMenuPages(List<ChatMessageContextMenuElement> elements) {
  if (elements.length <= _kMenuPageSize) {
    return [_MenuPage(elements: elements)];
  }

  final pages = <_MenuPage>[];
  var elementIndex = 0;
  var isFirstPage = true;

  while (elementIndex < elements.length) {
    final remaining = elements.length - elementIndex;
    final hasPrevious = !isFirstPage;
    final isFinalPage = hasPrevious && remaining <= _kMenuPageSize - 1;
    final hasNext = !isFinalPage;
    final reservedSlots = (hasPrevious ? 1 : 0) + (hasNext ? 1 : 0);
    final contentSlots = _kMenuPageSize - reservedSlots;
    final visibleCount = remaining.clamp(1, contentSlots);

    pages.add(
      _MenuPage(
        elements: elements
            .skip(elementIndex)
            .take(visibleCount)
            .toList(growable: false),
        hasPrevious: hasPrevious,
        hasNext: hasNext && elementIndex + visibleCount < elements.length,
      ),
    );

    elementIndex += visibleCount;
    isFirstPage = false;
  }

  return pages;
}

class _MenuPage {
  const _MenuPage({
    required this.elements,
    this.hasPrevious = false,
    this.hasNext = false,
  });

  final List<ChatMessageContextMenuElement> elements;
  final bool hasPrevious;
  final bool hasNext;
}

class _ReactionRow extends StatelessWidget {
  const _ReactionRow({
    super.key,
    required this.row,
    required this.surface,
    required this.activeColor,
    required this.disabledColor,
  });

  final ChatMessageReactionRow row;
  final Color surface;
  final Color activeColor;
  final Color disabledColor;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: surface,
        borderRadius: BorderRadius.circular(12),
      ),
      child: SizedBox(
        height: 49,
        child: Row(
          children: [
            for (final reaction in row.reactions)
              Expanded(
                child: _ReactionButton(
                  reaction: reaction,
                  rowEnabled: row.enabled,
                ),
              ),
            if (row.showAddReaction)
              Expanded(
                child: _SvgHoverButton(
                  asset: 'assets/icons/sub_menu/plus_icon.svg',
                  semanticLabel: 'More reactions',
                  onPressed: row.enabled ? row.onAddReaction : null,
                  enabled: row.enabled && row.onAddReaction != null,
                  activeColor: activeColor,
                  disabledColor: disabledColor,
                  size: 40,
                  iconSize: 31,
                ),
              ),
          ],
        ),
      ),
    );
  }
}

class _ReactionButton extends StatefulWidget {
  const _ReactionButton({required this.reaction, required this.rowEnabled});

  final ChatMessageQuickReaction reaction;
  final bool rowEnabled;

  @override
  State<_ReactionButton> createState() => _ReactionButtonState();
}

class _ReactionButtonState extends State<_ReactionButton> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final enabled =
        widget.rowEnabled &&
        widget.reaction.enabled &&
        widget.reaction.onPressed != null;
    final opacity = !enabled ? 0.28 : (_hovered ? 1.0 : 0.72);

    return Center(
      child: SizedBox.square(
        dimension: 40,
        child: Semantics(
          button: true,
          label: widget.reaction.semanticLabel,
          enabled: enabled,
          child: MouseRegion(
            onEnter: (_) {
              if (enabled) setState(() => _hovered = true);
            },
            onExit: (_) {
              if (_hovered) setState(() => _hovered = false);
            },
            child: IconButton(
              tooltip: widget.reaction.semanticLabel,
              onPressed: enabled ? widget.reaction.onPressed : null,
              icon: Opacity(opacity: opacity, child: widget.reaction.child),
              iconSize: 30,
              padding: EdgeInsets.zero,
              constraints: const BoxConstraints.tightFor(width: 40, height: 40),
            ),
          ),
        ),
      ),
    );
  }
}

class _MessageAction extends StatelessWidget {
  const _MessageAction({
    super.key,
    required this.action,
    required this.activeColor,
    required this.disabledColor,
  });

  final ChatMessageContextMenuAction action;
  final Color activeColor;
  final Color disabledColor;

  @override
  Widget build(BuildContext context) {
    final enabled = action.enabled && action.onPressed != null;
    return _HoverBuilder(
      enabled: enabled,
      builder: (context, hovered) {
        final color = !enabled
            ? disabledColor
            : (hovered ? activeColor : _kNormalColor);

        return SizedBox(
          height: 49,
          width: double.infinity,
          child: TextButton.icon(
            style: TextButton.styleFrom(
              alignment: Alignment.centerLeft,
              foregroundColor: color,
              disabledForegroundColor: color,
              padding: const EdgeInsets.symmetric(horizontal: 12),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
            ),
            onPressed: enabled ? action.onPressed : null,
            icon: SizedBox(
              width: 27,
              height: 27,
              child: _MenuSvg(asset: action.iconAsset, color: color),
            ),
            label: Padding(
              padding: const EdgeInsets.only(left: 12),
              child: Text(
                action.label,
                style: const TextStyle(
                  fontFamily: 'Roboto',
                  fontSize: 16,
                  fontWeight: FontWeight.w600,
                  letterSpacing: 1.5,
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

class _NavigationRow extends StatelessWidget {
  const _NavigationRow({
    super.key,
    required this.asset,
    required this.semanticLabel,
    required this.onPressed,
    required this.activeColor,
    required this.disabledColor,
  });

  final String asset;
  final String semanticLabel;
  final VoidCallback onPressed;
  final Color activeColor;
  final Color disabledColor;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 49,
      child: Center(
        child: _SvgHoverButton(
          asset: asset,
          semanticLabel: semanticLabel,
          onPressed: onPressed,
          enabled: true,
          activeColor: activeColor,
          disabledColor: disabledColor,
          size: 42,
          iconSize: 35,
        ),
      ),
    );
  }
}

class _SvgHoverButton extends StatelessWidget {
  const _SvgHoverButton({
    required this.asset,
    required this.semanticLabel,
    required this.onPressed,
    required this.enabled,
    required this.activeColor,
    required this.disabledColor,
    required this.size,
    required this.iconSize,
  });

  final String asset;
  final String semanticLabel;
  final VoidCallback? onPressed;
  final bool enabled;
  final Color activeColor;
  final Color disabledColor;
  final double size;
  final double iconSize;

  @override
  Widget build(BuildContext context) {
    return _HoverBuilder(
      enabled: enabled,
      builder: (context, hovered) {
        final color = !enabled
            ? disabledColor
            : (hovered ? activeColor : _kNormalColor);

        return SizedBox.square(
          dimension: size,
          child: IconButton(
            tooltip: semanticLabel,
            onPressed: enabled ? onPressed : null,
            icon: SizedBox.square(
              dimension: iconSize,
              child: _MenuSvg(asset: asset, color: color),
            ),
            padding: EdgeInsets.zero,
            constraints: BoxConstraints.tightFor(width: size, height: size),
          ),
        );
      },
    );
  }
}

class _HoverBuilder extends StatefulWidget {
  const _HoverBuilder({required this.enabled, required this.builder});

  final bool enabled;
  final Widget Function(BuildContext context, bool hovered) builder;

  @override
  State<_HoverBuilder> createState() => _HoverBuilderState();
}

class _HoverBuilderState extends State<_HoverBuilder> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) {
        if (widget.enabled) setState(() => _hovered = true);
      },
      onExit: (_) {
        if (_hovered) setState(() => _hovered = false);
      },
      child: widget.builder(context, _hovered),
    );
  }
}

class _MenuSvg extends StatelessWidget {
  const _MenuSvg({required this.asset, required this.color});

  final String asset;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      asset,
      package: _kComponentsPackage,
      fit: BoxFit.contain,
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
    );
  }
}
