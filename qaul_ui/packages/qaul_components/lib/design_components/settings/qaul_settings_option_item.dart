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
    final isActive = widget.selected || _isHovered;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(2),
          hoverColor: qaulSettingsHoverColor(context),
          onHover: (hovered) => setState(() => _isHovered = hovered),
          onTap: widget.onTap,
          child: SizedBox(
            height: 48,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 28),
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
                        letterSpacing: 1.8,
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
