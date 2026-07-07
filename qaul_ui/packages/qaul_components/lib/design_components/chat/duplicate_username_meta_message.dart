import 'package:flutter/material.dart';

import 'chat_meta_text_style.dart';

/// Centered group-room metadata shown when a joiner's name collides with an
/// existing member.
///
/// Copy is supplied by the caller so the widget stays localization-agnostic.
/// The action line is tappable only when [onEditPressed] is provided.
class DuplicateUsernameMetaMessage extends StatelessWidget {
  const DuplicateUsernameMetaMessage({
    super.key,
    required this.prefix,
    required this.emphasizedName,
    required this.actionLabel,
    this.onEditPressed,
  });

  /// Localized text before the emphasized name, e.g. "Group member Alice was
  /// renamed to".
  final String prefix;

  /// The disambiguated display name, rendered bold.
  final String emphasizedName;

  /// Localized action label, e.g. "Edit usernames".
  final String actionLabel;

  /// Tapped to edit the group's usernames. When null the action renders as
  /// plain (non-interactive) text.
  final VoidCallback? onEditPressed;

  @override
  Widget build(BuildContext context) {
    final base = chatMetaTextStyle(context);
    final actionStyle = base.copyWith(decoration: TextDecoration.underline);

    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text.rich(
            TextSpan(
              style: base,
              children: [
                TextSpan(text: '$prefix '),
                TextSpan(
                  text: emphasizedName,
                  style: base.copyWith(fontWeight: FontWeight.w700),
                ),
              ],
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 4),
          if (onEditPressed != null)
            GestureDetector(
              onTap: onEditPressed,
              child: Text(
                actionLabel,
                style: actionStyle,
                textAlign: TextAlign.center,
              ),
            )
          else
            Text(actionLabel, style: actionStyle, textAlign: TextAlign.center),
        ],
      ),
    );
  }
}
