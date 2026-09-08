import 'package:flutter/material.dart';

import 'qaul_settings_tokens.dart';

class QaulSettingsOptionItem extends StatefulWidget {
  const QaulSettingsOptionItem({
    super.key,
    required this.label,
    required this.selected,
    required this.onTap,
  });

  final String label;
  final bool selected;
  final VoidCallback onTap;

  @override
  State<QaulSettingsOptionItem> createState() => _QaulSettingsOptionItemState();
}

class _QaulSettingsOptionItemState extends State<QaulSettingsOptionItem> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final isIOS = Theme.of(context).platform == TargetPlatform.iOS;
    final verticalPadding = isIOS ? 2.0 : 4.0;
    final rowHeight = isIOS ? 44.0 : 48.0;
    final horizontalPadding = isIOS ? 20.0 : 28.0;
    final letterSpacing = isIOS ? 1.0 : 1.8;
    final isActive = widget.selected || _isHovered;

    return Padding(
      padding: EdgeInsets.symmetric(vertical: verticalPadding),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(2),
          hoverColor: qaulSettingsHoverColor(context),
          onHover: (hovered) => setState(() => _isHovered = hovered),
          onTap: widget.onTap,
          child: SizedBox(
            height: rowHeight,
            child: Padding(
              padding: EdgeInsets.symmetric(horizontal: horizontalPadding),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      widget.label,
                      style: Theme.of(context).textTheme.titleSmall?.copyWith(
                        fontWeight: FontWeight.w600,
                        color: qaulSettingsItemColor(
                          context,
                          selected: isActive,
                        ),
                        letterSpacing: letterSpacing,
                      ),
                    ),
                  ),
                  if (widget.selected)
                    Icon(
                      Icons.check,
                      size: 20,
                      color: qaulSettingsItemColor(context, selected: true),
                    ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
