import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthMoreTile extends StatelessWidget {
  const QaulAuthMoreTile({super.key, required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: kQaulAuthMoreRowHeight,
        child: const Padding(
          padding: EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              Icon(
                Icons.add_circle_outline,
                color: kQaulAuthSecondaryTextColor,
                size: kQaulAuthIconSize,
              ),
              SizedBox(width: kQaulAuthAvatarTextGap),
              Expanded(child: Text('more', style: kQaulAuthAccountTextStyle)),
              Icon(
                Icons.keyboard_arrow_down,
                color: kQaulAuthSecondaryTextColor,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
