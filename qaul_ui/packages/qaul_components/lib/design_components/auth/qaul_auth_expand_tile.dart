import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthExpandTile extends StatelessWidget {
  const QaulAuthExpandTile({super.key, required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: const SizedBox(
        height: kQaulAuthMoreRowHeight,
        child: Center(
          child: Icon(
            Icons.keyboard_arrow_down,
            color: kQaulAuthSecondaryTextColor,
            size: 32,
          ),
        ),
      ),
    );
  }
}
