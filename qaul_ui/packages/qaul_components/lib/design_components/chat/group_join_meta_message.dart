import 'package:flutter/material.dart';

/// Centered "user joined" metadata with the username emphasized.
class GroupJoinMetaMessage extends StatelessWidget {
  const GroupJoinMetaMessage({
    super.key,
    required this.userName,
    required this.joinedSuffix,
  });

  /// Display name of the member who joined (rendered bold).
  final String userName;

  /// Localized trailing phrase, e.g. " joined the group".
  final String joinedSuffix;

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

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text.rich(
        TextSpan(
          style: _metaStyle,
          children: [
            TextSpan(text: userName, style: _boldMetaStyle),
            TextSpan(text: joinedSuffix),
          ],
        ),
        textAlign: TextAlign.center,
      ),
    );
  }
}
