import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'qaul_settings_tokens.dart';

class QaulSettingsMenuItem extends StatefulWidget {
  const QaulSettingsMenuItem({
    super.key,
    required this.icon,
    required this.title,
    this.value,
    this.enabled = true,
    this.onTap,
  });

  final Widget icon;
  final String title;
  final String? value;
  final bool enabled;
  final VoidCallback? onTap;

  @override
  State<QaulSettingsMenuItem> createState() => _QaulSettingsMenuItemState();
}

class _QaulSettingsMenuItemState extends State<QaulSettingsMenuItem> {
  bool _isHovered = false;
  bool _suppressHoverUntilExit = false;

  @override
  Widget build(BuildContext context) {
    final color = qaulSettingsItemColor(context, selected: _isHovered);
    final textStyle = Theme.of(context).textTheme.titleSmall?.copyWith(
      color: color,
      fontWeight: FontWeight.w600,
      letterSpacing: 1.8,
    );

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(2),
          hoverColor: qaulSettingsHoverColor(context),
          onHover: (hovered) {
            setState(() {
              if (!hovered) {
                _suppressHoverUntilExit = false;
                _isHovered = false;
                return;
              }

              _isHovered = !_suppressHoverUntilExit;
            });
          },
          onTap: widget.enabled
              ? () {
                  setState(() {
                    _suppressHoverUntilExit = true;
                    _isHovered = false;
                  });
                  widget.onTap?.call();
                }
              : null,
          child: LayoutBuilder(
            builder: (context, constraints) {
              final valueWidth = constraints.maxWidth < 560 ? 160.0 : 300.0;

              return SizedBox(
                height: 56,
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 28),
                  child: Row(
                    children: [
                      SizedBox(
                        width: 48,
                        child: Align(
                          alignment: Alignment.center,
                          child: SizedBox.square(
                            dimension: 36,
                            child: Center(
                              child: IconTheme(
                                data: IconThemeData(color: color, size: 36),
                                child: widget.icon,
                              ),
                            ),
                          ),
                        ),
                      ),
                      const SizedBox(width: 18),
                      Expanded(
                        child: Text(
                          widget.title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: textStyle,
                        ),
                      ),
                      SizedBox(
                        width: valueWidth,
                        child: widget.value == null
                            ? const SizedBox.shrink()
                            : Text(
                                widget.value!,
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                                textAlign: TextAlign.end,
                                style: textStyle,
                              ),
                      ),
                      SizedBox(
                        width: 44,
                        child: widget.onTap == null
                            ? const SizedBox.shrink()
                            : Align(
                                alignment: Alignment.centerRight,
                                child: SvgPicture.asset(
                                  'assets/icons/arrow_right.svg',
                                  package: 'qaul_components',
                                  width: 9.206,
                                  height: 18.407,
                                  colorFilter: ColorFilter.mode(
                                    color,
                                    BlendMode.srcIn,
                                  ),
                                ),
                              ),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
