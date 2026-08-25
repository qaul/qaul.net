import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthActionRow extends StatelessWidget {
  const QaulAuthActionRow({
    super.key,
    required this.label,
    this.icon,
    this.leading,
    this.value,
    this.trailing,
    this.labelColor,
    this.onTap,
  });

  final String label;
  final IconData? icon;
  final Widget? leading;
  final String? value;
  final Widget? trailing;
  final Color? labelColor;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: kQaulAuthSectionHeaderHeight,
        child: Row(
          children: [
            SizedBox(
              width: kQaulAuthIconSize + 17,
              child:
                  leading ??
                  Icon(
                    icon,
                    color: kQaulAuthSecondaryTextColor,
                    size: kQaulAuthIconSize,
                  ),
            ),
            Expanded(
              child: Text(
                label,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: kQaulAuthLabelTextStyle.copyWith(color: labelColor),
              ),
            ),
            if (value != null)
              Flexible(
                child: Text(
                  value!,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.end,
                  style: kQaulAuthSecondaryTextStyle,
                ),
              ),
            if (trailing != null) ...[const SizedBox(width: 8), trailing!],
          ],
        ),
      ),
    );
  }
}
