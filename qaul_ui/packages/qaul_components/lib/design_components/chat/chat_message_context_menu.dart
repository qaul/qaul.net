import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

const _kComponentsPackage = 'qaul_components';
const _kReplyIcon = 'assets/icons/sub_menu/reply_arrow.svg';
const _kForwardIcon = 'assets/icons/sub_menu/forward_arrow.svg';
const _kEditIcon = 'assets/icons/sub_menu/edit.svg';

/// A quick reaction displayed at the top of a [ChatMessageContextMenu].
///
/// [child] is intentionally a widget so callers can supply emoji, an icon, or
/// an application-specific reaction asset without coupling the menu to a
/// reaction model.
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

/// Contextual actions for a selected chat message.
///
/// This widget owns only the menu presentation and callback wiring. Position it
/// with an [Overlay], [Stack], or any other parent appropriate to the chat UI.
class ChatMessageContextMenu extends StatelessWidget {
  const ChatMessageContextMenu({
    super.key,
    this.quickReactions = const [],
    this.onAddReaction,
    this.showAddReaction = true,
    this.addReactionEnabled = true,
    this.onReply,
    this.onForward,
    this.onEdit,
    this.onDismiss,
    this.showReply = true,
    this.showForward = true,
    this.showEdit = true,
    this.replyEnabled = true,
    this.forwardEnabled = true,
    this.editEnabled = true,
    this.showDismiss = true,
    this.dismissEnabled = true,
  });

  final List<ChatMessageQuickReaction> quickReactions;
  final VoidCallback? onAddReaction;
  final bool showAddReaction;
  final bool addReactionEnabled;

  final VoidCallback? onReply;
  final VoidCallback? onForward;
  final VoidCallback? onEdit;
  final VoidCallback? onDismiss;
  final bool showReply;
  final bool showForward;
  final bool showEdit;
  final bool replyEnabled;
  final bool forwardEnabled;
  final bool editEnabled;
  final bool showDismiss;
  final bool dismissEnabled;

  static const double width = 216;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final surface = isDark ? const Color(0xFF282828) : const Color(0xFFF1F1F1);
    final reactionSurface = isDark
        ? const Color(0xFF5A5A5A)
        : const Color(0xFFE0E0E0);
    final foreground = isDark ? Colors.white : const Color(0xFF252525);

    final actions = <Widget>[
      if (showReply)
        _MessageAction(
          label: 'Reply',
          iconAsset: _kReplyIcon,
          onPressed: replyEnabled ? onReply : null,
          foreground: foreground,
        ),
      if (showForward)
        _MessageAction(
          label: 'Forward',
          iconAsset: _kForwardIcon,
          onPressed: forwardEnabled ? onForward : null,
          foreground: foreground,
        ),
      if (showEdit)
        _MessageAction(
          label: 'Edit',
          iconAsset: _kEditIcon,
          onPressed: editEnabled ? onEdit : null,
          foreground: foreground,
        ),
    ];

    return Material(
      color: surface,
      elevation: 8,
      shadowColor: Colors.black.withValues(alpha: 0.35),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(18),
        side: const BorderSide(color: Color(0xFF999999), width: 1),
      ),
      clipBehavior: Clip.antiAlias,
      child: SizedBox(
        width: width,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(10, 12, 10, 14),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (quickReactions.isNotEmpty || showAddReaction) ...[
                DecoratedBox(
                  decoration: BoxDecoration(
                    color: reactionSurface,
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: SizedBox(
                    height: 49,
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        for (final reaction in quickReactions)
                          Expanded(child: _ReactionButton(reaction: reaction)),
                        if (showAddReaction)
                          Expanded(
                            child: Center(
                              child: SizedBox.square(
                                dimension: 40,
                                child: IconButton(
                                  tooltip: 'More reactions',
                                  onPressed: addReactionEnabled
                                      ? onAddReaction
                                      : null,
                                  icon: const Icon(Icons.add_circle_outline),
                                  color: foreground.withValues(alpha: 0.65),
                                  disabledColor: foreground.withValues(
                                    alpha: 0.25,
                                  ),
                                  iconSize: 32,
                                  padding: EdgeInsets.zero,
                                  constraints: const BoxConstraints.tightFor(
                                    width: 40,
                                    height: 40,
                                  ),
                                ),
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
                if (actions.isNotEmpty) const SizedBox(height: 8),
              ],
              ...actions,
              if (showDismiss) ...[
                const SizedBox(height: 4),
                _DismissButton(
                  onPressed: dismissEnabled ? onDismiss : null,
                  foreground: foreground,
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}

class _ReactionButton extends StatelessWidget {
  const _ReactionButton({required this.reaction});

  final ChatMessageQuickReaction reaction;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox.square(
        dimension: 40,
        child: Semantics(
          button: true,
          label: reaction.semanticLabel,
          enabled: reaction.enabled,
          child: IconButton(
            tooltip: reaction.semanticLabel,
            onPressed: reaction.enabled ? reaction.onPressed : null,
            icon: reaction.child,
            iconSize: 30,
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints.tightFor(width: 40, height: 40),
          ),
        ),
      ),
    );
  }
}

class _MessageAction extends StatefulWidget {
  const _MessageAction({
    required this.label,
    required this.iconAsset,
    required this.onPressed,
    required this.foreground,
  });

  final String label;
  final String iconAsset;
  final VoidCallback? onPressed;
  final Color foreground;

  @override
  State<_MessageAction> createState() => _MessageActionState();
}

class _MessageActionState extends State<_MessageAction> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final enabled = widget.onPressed != null;
    final color = enabled && _hovered
        ? widget.foreground
        : const Color(0xFF999999);

    return SizedBox(
      height: 49,
      width: double.infinity,
      child: TextButton.icon(
        style: TextButton.styleFrom(
          alignment: Alignment.centerLeft,
          foregroundColor: color,
          disabledForegroundColor: color,
          padding: const EdgeInsets.symmetric(horizontal: 12),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        ),
        onPressed: widget.onPressed,
        onHover: (hovered) {
          if (_hovered == hovered) return;
          setState(() => _hovered = hovered);
        },
        icon: SizedBox(
          width: 27,
          height: 27,
          child: SvgPicture.asset(
            widget.iconAsset,
            package: _kComponentsPackage,
            fit: BoxFit.contain,
            colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
          ),
        ),
        label: Padding(
          padding: const EdgeInsets.only(left: 12),
          child: Text(
            widget.label,
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

class _DismissButton extends StatefulWidget {
  const _DismissButton({required this.onPressed, required this.foreground});

  final VoidCallback? onPressed;
  final Color foreground;

  @override
  State<_DismissButton> createState() => _DismissButtonState();
}

class _DismissButtonState extends State<_DismissButton> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final enabled = widget.onPressed != null;
    final color = enabled && _hovered
        ? widget.foreground
        : const Color(0xFF999999);

    return Semantics(
      button: true,
      label: 'Close menu',
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: IconButton(
          tooltip: 'Close menu',
          onPressed: widget.onPressed,
          icon: Container(
            width: 42,
            height: 42,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              border: Border.all(color: color, width: 2.5),
            ),
            child: Icon(Icons.keyboard_arrow_down, color: color, size: 34),
          ),
          iconSize: 42,
          padding: EdgeInsets.zero,
        ),
      ),
    );
  }
}
