import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthAccountTile extends StatelessWidget {
  const QaulAuthAccountTile({
    super.key,
    required this.avatar,
    required this.name,
    required this.onTap,
  });

  final Widget avatar;
  final String name;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: kQaulAuthAccountRowHeight,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              avatar,
              const SizedBox(width: kQaulAuthAvatarTextGap),
              Expanded(
                child: Text(
                  name,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: kQaulAuthAccountTextStyle,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
