import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

/// Centered group-room metadata shown when a joiner's name collides with an existing member.
///
/// Copy is supplied by the caller so the widget stays localization-agnostic.
class DuplicateUsernameMetaMessage extends StatelessWidget {
  const DuplicateUsernameMetaMessage({
    super.key,
    required this.preamble,
    required this.baseName,
    required this.middle,
    required this.disambiguatedName,
    required this.actionLabel,
    this.onEditUserNames,
  });

  /// Text before [baseName], e.g. "Group member ".
  final String preamble;

  final String baseName;

  /// Text between [baseName] and the emphasized [disambiguatedName].
  final String middle;

  final String disambiguatedName;
  final String actionLabel;
  final VoidCallback? onEditUserNames;

  static const _metaStyle = TextStyle(
    fontSize: 12,
    height: 1.2,
    color: Colors.grey,
  );

  static const _boldMetaStyle = TextStyle(
    fontSize: 12,
    height: 1.2,
    fontWeight: FontWeight.w700,
    color: Colors.grey,
  );

  static const _actionStyle = TextStyle(
    fontSize: 12,
    height: 1.2,
    color: Colors.grey,
    decoration: TextDecoration.underline,
  );

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text.rich(
            TextSpan(
              style: _metaStyle,
              children: [
                TextSpan(text: preamble),
                TextSpan(text: baseName),
                TextSpan(text: middle),
                TextSpan(text: disambiguatedName, style: _boldMetaStyle),
              ],
            ),
            textAlign: TextAlign.center,
          ),
          if (onEditUserNames != null) ...[
            const SizedBox(height: 4),
            Text.rich(
              TextSpan(
                style: _actionStyle,
                text: actionLabel,
                recognizer: TapGestureRecognizer()..onTap = onEditUserNames,
              ),
            ),
          ] else
            Text(actionLabel, style: _actionStyle, textAlign: TextAlign.center),
        ],
      ),
    );
  }
}
