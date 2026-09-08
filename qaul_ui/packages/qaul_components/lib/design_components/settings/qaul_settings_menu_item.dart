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
    final isIOS = Theme.of(context).platform == TargetPlatform.iOS;
    final rowHeight = isIOS ? 48.0 : 56.0;
    final horizontalPadding = isIOS ? 20.0 : 28.0;
    final iconBoxSize = isIOS ? 30.0 : 36.0;
    final iconSize = isIOS ? 28.0 : 36.0;
    final gap = isIOS ? 14.0 : 18.0;
    final trailingWidth = isIOS ? 32.0 : 44.0;
    final verticalPadding = isIOS ? 2.0 : 4.0;
    final letterSpacing = isIOS ? 1.0 : 1.8;
    final color = qaulSettingsItemColor(context, selected: _isHovered);
    final textStyle = Theme.of(context).textTheme.titleSmall?.copyWith(
      color: color,
      fontWeight: FontWeight.w600,
      letterSpacing: letterSpacing,
    );

    return Padding(
      padding: EdgeInsets.symmetric(vertical: verticalPadding),
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
              final hasValue = widget.value != null;

              return SizedBox(
                height: rowHeight,
                child: Padding(
                  padding: EdgeInsets.symmetric(horizontal: horizontalPadding),
                  child: Row(
                    children: [
                      SizedBox(
                        width: iconBoxSize,
                        child: Align(
                          alignment: Alignment.center,
                          child: SizedBox.square(
                            dimension: iconBoxSize,
                            child: Center(
                              child: IconTheme(
                                data: IconThemeData(
                                  color: color,
                                  size: iconSize,
                                ),
                                child: widget.icon,
                              ),
                            ),
                          ),
                        ),
                      ),
                      SizedBox(width: gap),
                      Expanded(
                        child: Text(
                          widget.title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: textStyle,
                        ),
                      ),
                      if (hasValue)
                        SizedBox(
                          width: valueWidth,
                          child: Text(
                            widget.value!,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            textAlign: TextAlign.end,
                            style: textStyle,
                          ),
                        ),
                      SizedBox(
                        width: trailingWidth,
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
