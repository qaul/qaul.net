import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import '../../l10n/qaul_components_localizations.dart';
import '../../l10n/qaul_components_localizations_en.dart';

const _kComponentsPackage = 'qaul_components';
const _kNormalColor = Color(0xFF999999);
const _kDisabledDarkColor = Color(0xFF5F5F5F);
const _kDisabledLightColor = Color(0xFFC7C7C7);
const _kMenuPageSize = 5;
const _kRowHeight = 49.0;

abstract final class ChatMessageContextMenuStyle {
  static const quickReactionFontSize = 27.0;
}

/// Asset paths for the actions supplied with [ChatMessageContextMenu].
abstract final class ChatMessageContextMenuIcons {
  static const reply = 'assets/icons/sub_menu/reply_arrow.svg';
  static const forward = 'assets/icons/sub_menu/forward_arrow.svg';
  static const edit = 'assets/icons/sub_menu/edit.svg';
  static const info = 'assets/icons/sub_menu/info.svg';
  static const share = 'assets/icons/sub_menu/share.svg';
  static const copy = 'assets/icons/sub_menu/copy.svg';
  static const delete = 'assets/icons/sub_menu/delete.svg';
  static const previousPage = 'assets/icons/sub_menu/arrow-up.svg';
  static const nextPage = 'assets/icons/sub_menu/arrow-down.svg';
  static const addReaction = 'assets/icons/sub_menu/plus_icon.svg';
}

/// Resolves component localizations, falling back to English when the host
/// app's active locale is outside this package's supported set.
QaulComponentsLocalizations _l10n(BuildContext context) =>
    QaulComponentsLocalizations.of(context) ?? QaulComponentsLocalizationsEn();

/// The actions this package ships translations for.
enum ChatMessageContextMenuBuiltInAction {
  reply,
  forward,
  edit;

  String label(QaulComponentsLocalizations l10n) => switch (this) {
    reply => l10n.chatMenuReply,
    forward => l10n.chatMenuForward,
    edit => l10n.chatMenuEdit,
  };
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
///
/// To leave an element out of the menu, omit it from
/// [ChatMessageContextMenu.elements] rather than rendering it conditionally.
sealed class ChatMessageContextMenuElement {
  const ChatMessageContextMenuElement({this.enabled = true});

  /// Disabled elements remain visible but cannot be activated or highlighted.
  final bool enabled;
}

/// A single menu line containing quick reactions and an optional add button.
class ChatMessageReactionRow extends ChatMessageContextMenuElement {
  const ChatMessageReactionRow({
    required this.reactions,
    this.onAddReaction,
    this.showAddReaction = true,
    super.enabled,
  });

  final List<ChatMessageQuickReaction> reactions;
  final VoidCallback? onAddReaction;
  final bool showAddReaction;
}

/// A single labelled action line in a [ChatMessageContextMenu].
///
/// Built-in actions are labelled from this package's translations; any other
/// action carries the caller's own [label] text.
class ChatMessageContextMenuAction extends ChatMessageContextMenuElement {
  const ChatMessageContextMenuAction({
    required this.id,
    required String label,
    required this.iconAsset,
    this.onPressed,
    super.enabled,
  }) : _label = label,
       _builtIn = null;

  const ChatMessageContextMenuAction.reply({this.onPressed, super.enabled})
    : id = 'reply',
      _label = null,
      _builtIn = ChatMessageContextMenuBuiltInAction.reply,
      iconAsset = ChatMessageContextMenuIcons.reply;

  const ChatMessageContextMenuAction.forward({this.onPressed, super.enabled})
    : id = 'forward',
      _label = null,
      _builtIn = ChatMessageContextMenuBuiltInAction.forward,
      iconAsset = ChatMessageContextMenuIcons.forward;

  const ChatMessageContextMenuAction.edit({this.onPressed, super.enabled})
    : id = 'edit',
      _label = null,
      _builtIn = ChatMessageContextMenuBuiltInAction.edit,
      iconAsset = ChatMessageContextMenuIcons.edit;

  final String id;
  final String iconAsset;
  final VoidCallback? onPressed;

  /// Exactly one of these is set: the constructors keep them mutually
  /// exclusive, so [label] never has to guess.
  final String? _label;
  final ChatMessageContextMenuBuiltInAction? _builtIn;

  String label(QaulComponentsLocalizations l10n) {
    final builtIn = _builtIn;
    return builtIn == null ? _label! : builtIn.label(l10n);
  }
}

/// Resolves menu foreground colors from the interaction state of a button.
class _MenuPalette {
  _MenuPalette(Brightness brightness) : _isDark = brightness == Brightness.dark;

  final bool _isDark;

  Color get active => _isDark ? Colors.white : const Color(0xFF252525);
  Color get disabled => _isDark ? _kDisabledDarkColor : _kDisabledLightColor;
  Color get surface =>
      _isDark ? const Color(0xFF282828) : const Color(0xFFF1F1F1);
  Color get reactionSurface =>
      _isDark ? const Color(0xFF5A5A5A) : const Color(0xFFE0E0E0);

  /// Foreground for any menu button: dim when disabled, bright when hovered.
  WidgetStateProperty<Color> get foreground =>
      WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.disabled)) return disabled;
        if (states.contains(WidgetState.hovered)) return active;
        return _kNormalColor;
      });
}

/// Contextual actions for a selected chat message.
///
/// The menu owns presentation and local pagination only. It has no knowledge
/// of message storage, navigation, or backend behavior. A parent can position
/// it with an [Overlay], [Stack], or any other appropriate layout.
class ChatMessageContextMenu extends StatefulWidget {
  const ChatMessageContextMenu({super.key, required this.elements});

  final List<ChatMessageContextMenuElement> elements;

  static const double width = 200;

  @override
  State<ChatMessageContextMenu> createState() => _ChatMessageContextMenuState();
}

class _ChatMessageContextMenuState extends State<ChatMessageContextMenu> {
  int _pageIndex = 0;

  @override
  Widget build(BuildContext context) {
    final l10n = _l10n(context);
    final palette = _MenuPalette(Theme.of(context).brightness);
    final pages = _paginate(widget.elements);
    final pageIndex = _pageIndex.clamp(0, pages.length - 1);

    final rows = <Widget>[
      if (pageIndex > 0)
        _NavigationRow(
          key: const ValueKey('previous-page'),
          asset: ChatMessageContextMenuIcons.previousPage,
          semanticLabel: l10n.chatMenuPreviousPage,
          onPressed: () => setState(() => _pageIndex = pageIndex - 1),
          palette: palette,
        ),
      for (final element in pages[pageIndex])
        switch (element) {
          ChatMessageReactionRow() => _ReactionRow(
            key: const ValueKey('reaction-row'),
            row: element,
            palette: palette,
          ),
          ChatMessageContextMenuAction() => _MessageAction(
            key: ValueKey(element.id),
            action: element,
            palette: palette,
          ),
        },
      if (pageIndex < pages.length - 1)
        _NavigationRow(
          key: const ValueKey('next-page'),
          asset: ChatMessageContextMenuIcons.nextPage,
          semanticLabel: l10n.chatMenuNextPage,
          onPressed: () => setState(() => _pageIndex = pageIndex + 1),
          palette: palette,
        ),
    ];

    return Material(
      color: palette.surface,
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
            spacing: 2,
            children: rows,
          ),
        ),
      ),
    );
  }
}

/// Splits [elements] into pages of at most [_kMenuPageSize] rows, reserving a
/// row for each navigation arrow the page will show.
List<List<ChatMessageContextMenuElement>> _paginate(
  List<ChatMessageContextMenuElement> elements,
) {
  if (elements.length <= _kMenuPageSize) return [elements];

  final pages = <List<ChatMessageContextMenuElement>>[];
  for (var start = 0; start < elements.length;) {
    final remaining = elements.length - start;
    final isFirstPage = pages.isEmpty;
    // The last page only needs a back arrow; every other page also needs a
    // next arrow, and every page but the first needs a back arrow.
    final isLastPage = !isFirstPage && remaining <= _kMenuPageSize - 1;
    final slots = isLastPage
        ? remaining
        : _kMenuPageSize - (isFirstPage ? 1 : 2);
    pages.add(elements.sublist(start, start + slots));
    start += slots;
  }
  return pages;
}

class _ReactionRow extends StatelessWidget {
  const _ReactionRow({super.key, required this.row, required this.palette});

  final ChatMessageReactionRow row;
  final _MenuPalette palette;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: palette.reactionSurface,
        borderRadius: BorderRadius.circular(12),
      ),
      child: SizedBox(
        height: _kRowHeight,
        child: Row(
          children: [
            for (final reaction in row.reactions)
              Expanded(
                child: _ReactionButton(
                  reaction: reaction,
                  enabled:
                      row.enabled &&
                      reaction.enabled &&
                      reaction.onPressed != null,
                ),
              ),
            if (row.showAddReaction)
              Expanded(
                child: _IconOnlyButton(
                  asset: ChatMessageContextMenuIcons.addReaction,
                  semanticLabel: _l10n(context).chatMenuMoreReactions,
                  onPressed: row.enabled ? row.onAddReaction : null,
                  palette: palette,
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

class _ReactionButton extends StatelessWidget {
  const _ReactionButton({required this.reaction, required this.enabled});

  final ChatMessageQuickReaction reaction;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox.square(
        dimension: 40,
        child: Semantics(
          button: true,
          label: reaction.semanticLabel,
          enabled: enabled,
          child: _HoverBuilder(
            enabled: enabled,
            builder: (context, hovered) => IconButton(
              tooltip: reaction.semanticLabel,
              onPressed: enabled ? reaction.onPressed : null,
              icon: Opacity(
                opacity: !enabled ? 0.28 : (hovered ? 1.0 : 0.72),
                child: DefaultTextStyle.merge(
                  style: const TextStyle(
                    fontSize: ChatMessageContextMenuStyle.quickReactionFontSize,
                  ),
                  child: reaction.child,
                ),
              ),
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
    required this.palette,
  });

  final ChatMessageContextMenuAction action;
  final _MenuPalette palette;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: _kRowHeight,
      width: double.infinity,
      child: TextButton.icon(
        style:
            TextButton.styleFrom(
              alignment: Alignment.centerLeft,
              padding: const EdgeInsets.symmetric(horizontal: 12),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
            ).copyWith(
              foregroundColor: palette.foreground,
              iconColor: palette.foreground,
            ),
        onPressed: action.enabled ? action.onPressed : null,
        icon: SizedBox(
          width: 27,
          height: 27,
          child: _MenuSvg(asset: action.iconAsset),
        ),
        label: Padding(
          padding: const EdgeInsets.only(left: 12),
          child: Text(
            action.label(_l10n(context)),
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
  }
}

class _NavigationRow extends StatelessWidget {
  const _NavigationRow({
    super.key,
    required this.asset,
    required this.semanticLabel,
    required this.onPressed,
    required this.palette,
  });

  final String asset;
  final String semanticLabel;
  final VoidCallback onPressed;
  final _MenuPalette palette;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: _kRowHeight,
      child: Center(
        child: _IconOnlyButton(
          asset: asset,
          semanticLabel: semanticLabel,
          onPressed: onPressed,
          palette: palette,
          size: 42,
          iconSize: 35,
        ),
      ),
    );
  }
}

class _IconOnlyButton extends StatelessWidget {
  const _IconOnlyButton({
    required this.asset,
    required this.semanticLabel,
    required this.onPressed,
    required this.palette,
    required this.size,
    required this.iconSize,
  });

  final String asset;
  final String semanticLabel;
  final VoidCallback? onPressed;
  final _MenuPalette palette;
  final double size;
  final double iconSize;

  @override
  Widget build(BuildContext context) {
    return SizedBox.square(
      dimension: size,
      child: IconButton(
        tooltip: semanticLabel,
        onPressed: onPressed,
        style: ButtonStyle(iconColor: palette.foreground),
        icon: SizedBox.square(
          dimension: iconSize,
          child: _MenuSvg(asset: asset),
        ),
        padding: EdgeInsets.zero,
        constraints: BoxConstraints.tightFor(width: size, height: size),
      ),
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

/// Paints a menu icon in the foreground color resolved by its enclosing button.
class _MenuSvg extends StatelessWidget {
  const _MenuSvg({required this.asset});

  final String asset;

  @override
  Widget build(BuildContext context) {
    final color = IconTheme.of(context).color ?? _kNormalColor;
    return SvgPicture.asset(
      asset,
      package: _kComponentsPackage,
      fit: BoxFit.contain,
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
    );
  }
}
