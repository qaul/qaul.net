import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthSectionTitle extends StatelessWidget {
  const QaulAuthSectionTitle({
    super.key,
    this.icon,
    this.leading,
    required this.label,
  });

  final IconData? icon;
  final Widget? leading;
  final String label;

  @override
  Widget build(BuildContext context) {
    final titleIcon =
        leading ??
        Icon(
          icon,
          color: qaulAuthPrimaryTextColor(context),
          size: kQaulAuthIconSize,
        );

    return SizedBox(
      height: kQaulAuthSectionHeaderHeight,
      child: Row(
        children: [
          SizedBox(
            width: kQaulAuthIconSize + kQaulAuthAvatarTextGap,
            child: Center(child: titleIcon),
          ),
          Text(label, style: qaulAuthLabelTextStyle(context)),
        ],
      ),
    );
  }
}
