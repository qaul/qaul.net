import 'package:flutter/material.dart';

import 'chat_meta_text_style.dart';

/// Centered "user joined" metadata with the username emphasized.
///
/// Copy is supplied by the caller so the widget stays localization-agnostic.
class GroupJoinMetaMessage extends StatelessWidget {
  const GroupJoinMetaMessage({
    super.key,
    required this.userName,
    required this.joinedSuffix,
  });

  /// Display name of the member who joined (rendered bold).
  final String userName;

  /// Localized trailing phrase, e.g. "has joined the group".
  final String joinedSuffix;

  @override
  Widget build(BuildContext context) {
    final base = chatMetaTextStyle(context);
    return Center(
      child: Text.rich(
        TextSpan(
          style: base,
          children: [
            TextSpan(
              text: userName,
              style: base.copyWith(fontWeight: FontWeight.w700),
            ),
            TextSpan(text: ' $joinedSuffix'),
          ],
        ),
        textAlign: TextAlign.center,
      ),
    );
  }
}
