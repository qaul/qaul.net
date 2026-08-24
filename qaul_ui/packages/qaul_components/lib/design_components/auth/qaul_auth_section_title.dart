import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthSectionTitle extends StatelessWidget {
  const QaulAuthSectionTitle({
    super.key,
    required this.icon,
    required this.label,
  });

  final IconData icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: kQaulAuthSectionHeaderHeight,
      child: Row(
        children: [
          Icon(icon, color: kQaulAuthPrimaryTextColor, size: kQaulAuthIconSize),
          const SizedBox(width: kQaulAuthAvatarTextGap),
          Text(label, style: kQaulAuthLabelTextStyle),
        ],
      ),
    );
  }
}
